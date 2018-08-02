//! Core IPC primitives
//!
//! This module contains the low-level primitives used for IPC.
//!

// TODO: Separate HIPC from CMIF ? HIPC is the low-level IPC mechanism, whereas
// CMIF is the higher-level Command-based IPC protocol.

use core;
use core::mem;
use core::marker::PhantomData;
use arrayvec::{ArrayVec, Array};
use alloc::sync::Arc;
use byteorder::{LE};
use kernel::{KObject, Domain};
use bit_field::BitField;

use utils::{CursorWrite, CursorRead, div_ceil, hex_print};
use error::*;

// Fits in a QWORD
// TODO: Migrate bitfield_register_macro to a custom Derive
/// Represents the header of a packed IPC message, to be sent to the kernel for
/// processing. This is usually written in the first 0x100 byes of TLS, but
/// any memory locaction may be used, if used with `svcSendSyncRequestWithUserBuffer`.
#[repr(transparent)]
struct MsgPackedHdr(u64);

impl MsgPackedHdr {
    #[allow(dead_code)]
    pub fn get_ty(&self) -> u16 {
        self.0.get_bits(0..16) as u16
    }

    #[allow(dead_code)]
    pub fn set_ty(&mut self, n: u16) {
        self.0 = *self.0.set_bits(0..16, n as u64)
    }

    #[allow(dead_code)]
    pub fn get_num_x_descriptors(&self) -> u8 {
        self.0.get_bits(16..20) as u8
    }

    #[allow(dead_code)]
    pub fn set_num_x_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(16..20, n as u64)
    }

    #[allow(dead_code)]
    pub fn get_num_a_descriptors(&self) -> u8 {
        self.0.get_bits(20..24) as u8
    }

    #[allow(dead_code)]
    pub fn set_num_a_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(20..24, n as u64)
    }

    #[allow(dead_code)]
    pub fn get_num_b_descriptors(&self) -> u8 {
        self.0.get_bits(24..28) as u8
    }

    #[allow(dead_code)]
    pub fn set_num_b_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(24..28, n as u64)
    }

    #[allow(dead_code)]
    pub fn get_num_w_descriptors(&self) -> u8 {
        self.0.get_bits(28..32) as u8
    }

    #[allow(dead_code)]
    pub fn set_num_w_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(28..32, n as u64)
    }

    #[allow(dead_code)]
    pub fn get_raw_section_size(&self) -> u16 {
        self.0.get_bits(32..42) as u16
    }

    #[allow(dead_code)]
    pub fn set_raw_section_size(&mut self, n: u16) {
        self.0 = *self.0.set_bits(32..42, n as u64)
    }

    #[allow(dead_code)]
    pub fn get_c_descriptor_flags(&self) -> u8 {
        self.0.get_bits(42..46) as u8
    }

    #[allow(dead_code)]
    pub fn set_c_descriptor_flags(&mut self, n: u8) {
        self.0 = *self.0.set_bits(42..46, n as u64)
    }

    //pub fn get_unknown(&self) -> u16 {
    //    self.0.get_bits(52..63) as u16
    //}

    #[allow(dead_code)]
    pub fn get_enable_handle_descriptor(&self) -> bool {
        self.0.get_bit(63)
    }

    #[allow(dead_code)]
    pub fn set_enable_handle_descriptor(&mut self, n: bool) {
        self.0 = *self.0.set_bit(63, n)
    }
}

#[repr(transparent)]
struct HandleDescriptorHeader(u16);

impl HandleDescriptorHeader {
    pub fn get_send_pid(&self) -> bool {
        self.0.get_bit(0)
    }

    pub fn set_send_pid(&mut self, n: bool) {
        self.0 = *self.0.set_bit(0, n)
    }

    pub fn get_num_copy_handles(&self) -> u8 {
        self.0.get_bits(1..5) as u8
    }

    pub fn set_num_copy_handles(&mut self, n: u8) {
        self.0 = *self.0.set_bits(1..5, n as u16)
    }

    pub fn get_num_move_handles(&self) -> u8 {
        self.0.get_bits(5..9) as u8
    }

    pub fn set_num_move_handles(&mut self, n: u8) {
        self.0 = *self.0.set_bits(5..9, n as u16)
    }
}

#[repr(transparent)]
struct DomainMessageHeader(u32);

impl DomainMessageHeader {
    #[allow(dead_code)]
    pub fn get_command(&self) -> u8 {
        self.0.get_bits(0..8) as u8
    }

    pub fn set_command(&mut self, n: u8) {
        self.0 = *self.0.set_bits(0..8, n as u32);
    }

    #[allow(dead_code)]
    pub fn get_input_object_count(&self) -> u8 {
        self.0.get_bits(8..16) as u8
    }

    pub fn set_input_object_count(&mut self, n: u8) {
        self.0 = *self.0.set_bits(8..16, n as u32);
    }

    #[allow(dead_code)]
    pub fn get_data_len(&self) -> u16 {
        self.0.get_bits(16..32) as u16
    }

    pub fn set_data_len(&mut self, n: u16) {
        self.0 = *self.0.set_bits(16..32, n as u32);
    }
}

assert_eq_size!(AssertHandleDescriptorHeader; u16, HandleDescriptorHeader);

#[derive(Debug)]
pub enum MessageType {
    Request,
    Control,
    Unknown(u16)
}

#[derive(Debug, Clone, Copy)]
pub enum IPCBufferType {
   A { flags: u8 }, B { flags: u8 }, X /* { counter: u16 } */, C { has_u16_size: bool },
   AX, BC { unk_val: bool } // TODO: what other data, if any, should we return with these?
}

#[derive(Debug, Clone)]
pub struct IPCBuffer<'a> {
    // Address to the value
    addr: usize,
    // Size of the value
    size: usize,
    // Buffer type
    ty: u64,
    // Tie the buffer's lifetime to the value's !
    // This is very very very important, for the safety of this interface. It ensures that, as long as
    // this IPCBuffer exist, the value it references cannot be dropped.
    phantom: PhantomData<&'a ()>
}

impl<'a> IPCBuffer<'a> {
    fn null() -> IPCBuffer<'static> {
        IPCBuffer {
            addr: 0,
            size: 0,
            ty: 0,
            phantom: PhantomData
        }
    }
    pub fn from_mut_ref<T>(val: &'a mut T, ty: u64) -> IPCBuffer {
        // TODO: Verify type and val mutability
        IPCBuffer {
            addr: val as *mut T as usize,
            size: core::mem::size_of::<T>(),
            ty,
            phantom: PhantomData
        }
    }
    pub fn from_ref<T>(val: &'a T, ty: u64) -> IPCBuffer {
        // TODO: Verify type and val mutability
        IPCBuffer {
            addr: val as *const T as usize,
            size: core::mem::size_of::<T>(),
            ty,
            phantom: PhantomData
        }
    }
    pub fn from_slice<T>(val: &'a [T], ty: u64) -> IPCBuffer {
        // TODO: Verify type and val mutability
        IPCBuffer {
            addr: if val.len() == 0 { 0 } else { val.as_ptr() as usize },
            size: core::mem::size_of::<T>() * val.len(),
            ty,
            phantom: PhantomData
        }
    }
    pub fn from_mut_slice<T>(val: &'a mut [T], ty: u64) -> IPCBuffer {
        // TODO: Verify type and val mutability
        IPCBuffer {
            addr: if val.len() == 0 { 0 } else { val.as_ptr() as usize },
            size: core::mem::size_of::<T>() * val.len(),
            ty,
            phantom: PhantomData
        }
    }

    pub unsafe fn from_ptr_len<T>(val: *const T, len: usize, ty: u64) -> IPCBuffer<'static> {
        IPCBuffer {
            addr: val as usize,
            size: core::mem::size_of::<T>() * len,
            ty,
            phantom: PhantomData
        }
    }

    pub unsafe fn from_mut_ptr_len<T>(val: *mut T, len: usize, ty: u64) -> IPCBuffer<'static> {
        IPCBuffer {
            addr: val as usize,
            size: core::mem::size_of::<T>() * len,
            ty,
            phantom: PhantomData
        }
    }

    // Based on http://switchbrew.org/index.php?title=IPC_Marshalling#Official_marshalling_code
    fn buftype(&self) -> IPCBufferType {
        enum Direction { In, Out }
        enum Family { AB, XC }
        let direction = match self.ty & 0b11 {
            0b01 => Direction::In,
            0b10 => Direction::Out,
            _ => panic!("Invalid buffer type {}", self.ty)
        };

        // TODO: should we check this only when we actually need it?
        let flags = match (self.ty >> 6) & 0b11 {
            0b00 => 0,
            0b01 => 1,
            0b10 => 3,
            0b11 => panic!("Invalid buffer type {}", self.ty), /* TODO: Is this really invalid? */
            _ => unreachable!()
        };

        // if self.ty & !0xFF != 0 { panic!("Invalid buffer type {}", self.ty) }

        if self.ty & 0x20 == 0 {
            let family = match (self.ty >> 2) & 0b11 {
                0b01 => Family::AB,
                0b10 => Family::XC,
                _ => panic!("Invalid buffer type {}", self.ty)
            };

            match (direction, family) {
                (Direction::In, Family::AB) => IPCBufferType::A { flags },
                (Direction::Out, Family::AB) => IPCBufferType::B { flags },
                (Direction::In, Family::XC) => IPCBufferType::X,
                (Direction::Out, Family::XC) => IPCBufferType::C { has_u16_size: self.ty & 0x10 == 0 } /* TODO: Is that right ? */
            }
        } else {
            match direction {
                Direction::In => IPCBufferType::AX,
                Direction::Out => IPCBufferType::BC { unk_val: self.ty & 0x40 != 0 },
            }
        }
    }
}

// TODO: Oh how nice the world good be
//struct Request<'a, const BUFF_COUNT: usize, const COPY_HANDLES_COUNT: usize, const MOVE_HANDLES_COUNT: usize> {
//	ty: u16,
//	send_pid: u64,
//	buffers: [IPCBuffer<'a>; BUFF_COUNT],
//	copy_handles: [u32; COPY_HANDLES_COUNT],
//	move_handles: [u32; MOVE_HANDLES_COUNT],
//	// The data section is built in !
//	data: T,

//}

//#[derive(Debug)]
pub struct Request<'a, 'b, RAW, BUFF = [IPCBuffer<'a>; 0], COPY = [&'b KObject; 0], MOVE = [KObject; 0]>
where
    BUFF: Array<Item=IPCBuffer<'a>>,
    COPY: Array<Item=&'b KObject>,
    MOVE: Array<Item=KObject>
{
    ty: u16,
    send_pid: bool,
    buffers: ArrayVec<BUFF>,
    // x_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    // a_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    // b_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    // c_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    copy_handles: ArrayVec<COPY>,
    move_handles: ArrayVec<MOVE>,

    // The data section is built in !
    cmd_id: u64,
    // TODO: I really feel like this *ought* to be
    args: Option<RAW>
}

pub trait IRequest {
    fn pack(self, data: &mut [u8], domain_id: Option<u32>);
}

impl<'a, 'b, T: Clone, BUFF, COPY, MOVE> Request<'a, 'b, T, BUFF, COPY, MOVE>
where
    BUFF: Array<Item=IPCBuffer<'a>>,
    COPY: Array<Item=&'b KObject>,
    MOVE: Array<Item=KObject>
{
    pub fn new(id: u64) -> Self {
        Request {
            ty: 4,
            cmd_id: id,
            send_pid: false,
            buffers: ArrayVec::new(),
            // x_descriptors: ArrayVec::new(),
            // a_descriptors: ArrayVec::new(),
            // b_descriptors: ArrayVec::new(),
            // c_descriptors: ArrayVec::new(),
            copy_handles: ArrayVec::new(),
            move_handles: ArrayVec::new(),
            args: None
        }
    }

    pub fn ty(mut self, ty: MessageType) -> Self {
        self.ty = match ty {
            MessageType::Request => 4,
            MessageType::Control => 5,
            MessageType::Unknown(v) => v
        };
        self
    }

    pub fn send_pid(mut self) -> Self {
        self.send_pid = true;
        self
    }

    pub fn descriptor(mut self, buf: IPCBuffer<'a>) -> Self {
        buf.buftype(); // ignore result, validates the type though!
        self.buffers.push(buf);
        self
    }

    // TODO: Take an AsKObject
    pub fn copy_handle(mut self, hnd: &'b KObject) -> Self {
        self.copy_handles.push(hnd);
        self
    }

    // TODO: Take ref to raw ?
    pub fn args(mut self, args: T) -> Self {
        self.args = Some(args);
        self
    }

    pub fn show_packed(self, f: &mut core::fmt::Write, is_domain: bool) -> Self {
        // Let's make a copy. **WE NEED TO FORGET IT**
        // TODO: Maybe there's a cleaner way to unsafely make a copy without
        // transmuting ?
        let other_self : Self = Self {
            ty: self.ty,
            send_pid: self.send_pid,
            buffers: self.buffers.clone(),
            //x_descriptors: self.x_descriptors.clone(),
            //a_descriptors: self.a_descriptors.clone(),
            //b_descriptors: self.b_descriptors.clone(),
            //c_descriptors: self.c_descriptors.clone(),
            copy_handles: self.copy_handles.clone(),
            // This works because it gets forgotten.
            move_handles: self.move_handles.iter().map(|o| unsafe { KObject::new(o.as_raw_handle()) }).collect(),

            // The data section is built in !
            cmd_id: self.cmd_id,
            // TODO: I really feel like this *ought* to be
            args: self.args.clone()
        };
        let mut arr = [0; 0x100];
        other_self.pack(&mut arr, if is_domain { Some(0xff) } else { None });

        hex_print(&arr[..], f);
        self
    }
}

// TODO: Figure out a way to avoid T: Clone ?
// TODO: Maybe I should just store a *pointer* to T ?
impl<'a, 'b, T, BUFF, COPY, MOVE> IRequest for Request<'a, 'b, T, BUFF, COPY, MOVE>
where
    T: Clone,
    BUFF: Array<Item=IPCBuffer<'a>>,
    COPY: Array<Item=&'b KObject>,
    MOVE: Array<Item=KObject>
{
    // Write the data to an IPC buffer to be sent to the Switch OS.
    // TODO: If this is not sent, it can leak move handles!
    fn pack(self, data: &mut [u8], domain_id: Option<u32>) {
        let mut descriptor_counts = (/* X */0, /* A */0, /* B */0, /* C */0);
        for bufty in self.buffers.iter().map(|b| b.buftype()) {
            match bufty {
                IPCBufferType::X => descriptor_counts.0 += 1,
                IPCBufferType::A {flags: _} => descriptor_counts.1 += 1,
                IPCBufferType::B {flags: _} => descriptor_counts.2 += 1,
                IPCBufferType::C {has_u16_size: _} => descriptor_counts.3 += 1,
                IPCBufferType::AX => {
                    descriptor_counts.0 += 1; // X
                    descriptor_counts.1 += 1; // A
                },
                IPCBufferType::BC { unk_val: _ } => {
                    descriptor_counts.2 += 1; // B
                    descriptor_counts.3 += 1; // C
                }
            }
        }

        // TODO: Memset data first
        let mut cursor = CursorWrite::new(data);

        // Let's get a header from our data
        // TODO: There are certainly other checks necessary, such as alignment,
        // etc...
        //assert!(core::mem::size_of::<MsgPackedHdr>() < data.len());

        // Get the header.
        {
            let hdr = cursor.skip_write(core::mem::size_of::<MsgPackedHdr>());
            // Unsafely cast the header.
            let hdr = unsafe {
                (hdr.as_mut_ptr() as *mut MsgPackedHdr).as_mut().unwrap()
            };

            hdr.set_ty(self.ty);
            hdr.set_num_x_descriptors(descriptor_counts.0);
            hdr.set_num_a_descriptors(descriptor_counts.1);
            hdr.set_num_b_descriptors(descriptor_counts.2);
            hdr.set_num_w_descriptors(0);
            if descriptor_counts.3 == 0 {
                hdr.set_c_descriptor_flags(0);
            } else if descriptor_counts.3 == 1 {
                hdr.set_c_descriptor_flags(2);
            } else {
                hdr.set_c_descriptor_flags(2 + descriptor_counts.3 as u8);
            }

            // 0x10 = padding, 8 = sfci, 8 = cmdid, data = T
            let mut raw_section_size = 0x10 + 8 + 8 + core::mem::size_of::<T>();
            if domain_id.is_some() {
                // Domain Header.
                // TODO: Input ObjectIDs
                raw_section_size += 0x10;
            }
            // C descriptor u16 sizes
            raw_section_size += self.buffers.iter().filter(|v| v.ty & 0xF == 0xA && v.ty & 0x10 != 0).count() * 2;

            hdr.set_raw_section_size(div_ceil(raw_section_size as u64, 4) as u16);
            let enable_handle_descriptor = self.copy_handles.len() > 0 ||
                self.move_handles.len() > 0 || self.send_pid;
            hdr.set_enable_handle_descriptor(enable_handle_descriptor);
        }

        // First, write the handle descriptor
        if self.copy_handles.len() > 0 ||
            self.move_handles.len() > 0 || self.send_pid {

            // Handle Descriptor Header
            {
                let descriptor_hdr = cursor.skip_write(core::mem::size_of::<HandleDescriptorHeader>());
                let descriptor_hdr = unsafe {
                    (descriptor_hdr.as_mut_ptr() as *mut HandleDescriptorHeader).as_mut().unwrap()
                };

                // Write the header
                descriptor_hdr.set_num_copy_handles(self.copy_handles.len() as u8);
                descriptor_hdr.set_num_move_handles(self.move_handles.len() as u8);
                descriptor_hdr.set_send_pid(self.send_pid);
            }

            // Descriptor_hdr is 2 bytes, but everything is encoded on 4-bytes...
            cursor.skip_write(2);

            // Seek 8 if we have to send pid. We don't actually write the pid.
            if self.send_pid {
                cursor.skip_write(8);
            }

            // Write copy and move handles
            for hnd in self.copy_handles {
                cursor.write_u32::<LE>(hnd.as_raw_handle());
            }
            for hnd in self.move_handles {
                cursor.write_u32::<LE>(hnd.as_raw_handle());
                core::mem::forget(hnd);
            }
        }

        // X descriptors
        {
            let mut i = 0;
            for buf in self.buffers.iter() {
                let null_buf = IPCBuffer::null();
                let buf = match buf.buftype() {
                    IPCBufferType::X => buf,
                    IPCBufferType::AX => &null_buf,
                    _ => continue
                };

                assert!(buf.addr >> 39 == 0, "Invalid buffer address");
                assert!(buf.size >> 16 == 0, "Invalid buffer size");
                let num = i & 0b111000111111
                    | ((buf.addr >> 36) & 0b111) << 6
                    | ((buf.addr >> 32) & 0b1111) << 12
                    | buf.size << 16;
                cursor.write_u32::<LE>(num as u32);
                cursor.write_u32::<LE>((buf.addr & 0xFFFFFFFF) as u32);
                i += 1;
            }
        }

        // A descriptors
        for buf in self.buffers.iter() {
            let (buf, flags) = match buf.buftype() {
                IPCBufferType::A {flags} => (buf, flags),
                IPCBufferType::AX => (buf, 0),
                _ => continue
            };

            assert!(buf.addr >> 39 == 0, "Invalid buffer address");
            assert!(buf.size >> 35 == 0, "Invalid buffer size");
            assert!(buf.ty >> 8 == 0, "Invalid descriptor permission");

            cursor.write_u32::<LE>((buf.size & 0xFFFFFFFF) as u32);
            cursor.write_u32::<LE>((buf.addr & 0xFFFFFFFF) as u32);

            let num = flags as usize
                | ((buf.addr >> 36) & 0b111) << 2
                | ((buf.size >> 32) & 0b1111) << 24
                | ((buf.addr >> 32) & 0b1111) << 28;
            cursor.write_u32::<LE>(num as u32);
        }

        // B descriptors
        for buf in self.buffers.iter() {
            let (buf, flags) = match buf.buftype() {
                IPCBufferType::B {flags} => (buf, flags),
                IPCBufferType::BC {unk_val: _} => (buf, 0),
                _ => continue
            };
            assert!(buf.addr >> 39 == 0, "Invalid buffer address");
            assert!(buf.size >> 35 == 0, "Invalid buffer size");
            assert!(buf.ty >> 8 == 0, "Invalid descriptor permission");

            cursor.write_u32::<LE>((buf.size & 0xFFFFFFFF) as u32);
            cursor.write_u32::<LE>((buf.addr & 0xFFFFFFFF) as u32);

            let num = flags as usize
                | ((buf.addr >> 36) & 0b111) << 2
                | ((buf.size >> 32) & 0b1111) << 24
                | ((buf.addr >> 32) & 0b1111) << 28;
            cursor.write_u32::<LE>(num as u32);
        }

        // TODO: W descriptors would go there.

        // Align to 16-byte boundary
        let before_pad = ((cursor.pos() + (16 - 1)) & !(16 - 1)) - cursor.pos();
        cursor.skip_write(before_pad);

        if let Some(obj) = domain_id {
            {
                let hdr = cursor.skip_write(core::mem::size_of::<DomainMessageHeader>());
                let hdr = unsafe {
                    (hdr.as_mut_ptr() as *mut DomainMessageHeader).as_mut().unwrap()
                };
                hdr.set_command(1);
                hdr.set_input_object_count(0);
                hdr.set_data_len(core::mem::size_of::<T>() as u16 + 0x10);
            }
            cursor.write_u32::<LE>(obj);
            // Apparently this is some padding. :shrug:
            cursor.write_u64::<LE>(0);
        }
        cursor.write(b"SFCI\0\0\0\0");
        cursor.write_u64::<LE>(self.cmd_id);
        // TODO: Should blow up if that's not true. Alternatively: This should
        // not even be possible from an API perspective ?
        if let Some(args) = self.args {
            let raw_data = cursor.skip_write(core::mem::size_of::<T>());
            let raw_data = unsafe {
                (raw_data.as_mut_ptr() as *mut T).as_mut().unwrap()
            };
            *raw_data = args.clone();
        } else {
            if core::mem::size_of::<T>() != 0 {
                panic!("Called pack with arguments unset");
            }
        }

        // Write input object IDs. For now: none.

        // Total padding should be 0x10
        cursor.skip_write(0x10 - before_pad);


        // C descriptor u16 length list
        let mut i = 0;
        for buf in self.buffers.iter() {
            let nullbuf = IPCBuffer::null();
            let buf = match buf.buftype() {
                IPCBufferType::C { has_u16_size: true } => buf,
                IPCBufferType::BC { unk_val: _ } => &nullbuf,
                _ => continue
            };

            if buf.size >> 16 != 0 {
                panic!("Invalid buffer size {:x}", buf.size);
            }

            cursor.write_u16::<LE>((buf.size) as u16);
            i += 1;
        }

        // Align to u32
        if i % 2 == 1 {
            cursor.skip_write(2);
        }

        for buf in self.buffers.iter() {
            let nullbuf = IPCBuffer::null();
            let buf = match buf.buftype() {
                IPCBufferType::C { has_u16_size: _ } => buf,
                IPCBufferType::BC { unk_val: _ } => &nullbuf,
                _ => continue
            };

            assert_eq!(buf.addr >> 48, 0, "Invalid address {:x}", buf.addr);
            assert_eq!(buf.size >> 16, 0, "Invalid size {:x}", buf.size);

            cursor.write_u32::<LE>(buf.addr as u32);
            cursor.write_u32::<LE>((buf.addr >> 32) as u32 | (buf.size as u32) << 16);
        }
    }
}

// TODO: I could make RAW be a reference, if I'm OK with tying it to the TLS
// buffer's lifetime (somehow).
#[derive(Debug)]
pub struct Response<RAW> {
    domain_obj: Option<Arc<KObject>>,
    error: u64,
    pid: Option<u64>,
    handles: ArrayVec<[KObject; 32]>,
    objects: ArrayVec<[u32; 256]>, // TODO: Maybe it'd be fine to lower this below the theoretical limit?
    ret: RAW
}

impl<T: Clone> Response<T> {
    // TODO: Mark unpack as unsafe (for all the obvious reasons)
    pub fn unpack(data: &[u8], is_domain: Option<Arc<KObject>>) -> Result<Response<T>> {
        let mut this : Response<T> = Response {
            domain_obj: is_domain,
            error: 0,
            pid: None,
            handles: ArrayVec::new(),
            objects: ArrayVec::new(),
            ret: unsafe { mem::uninitialized() }
        };

        let cursor = CursorRead::new(data);

        let hdr = cursor.skip_read(core::mem::size_of::<MsgPackedHdr>());
        // Unsafely cast the header.
        let hdr = unsafe {
            (hdr.as_ptr() as *const MsgPackedHdr).as_ref().unwrap()
        };

        // TODO: What about control messages ?
        // TODO: What about buffers ??
        // Twili sends 0 here, Nintendo send 4.
        //assert_eq!(hdr.get_ty(), 4);

        // First, read the handle descriptor
        if hdr.get_enable_handle_descriptor() {
            let descriptor_hdr = cursor.skip_read(core::mem::size_of::<HandleDescriptorHeader>());
            let descriptor_hdr = unsafe {
                (descriptor_hdr.as_ptr() as *const HandleDescriptorHeader).as_ref().unwrap()
            };
            cursor.skip_read(2);

            if descriptor_hdr.get_send_pid() {
                // TODO
                let _pid = cursor.read_u64::<LE>();
            }
            for _ in 0..descriptor_hdr.get_num_copy_handles() {
                this.handles.push(unsafe { KObject::new(cursor.read_u32::<LE>()) });
            }
            for _ in 0..descriptor_hdr.get_num_move_handles() {
                this.handles.push(unsafe { KObject::new(cursor.read_u32::<LE>()) });
            }
        }

        // Then take care of the buffers
        for _ in 0..hdr.get_num_x_descriptors() {
            // skip 2 words
            cursor.read_u32::<LE>();
            cursor.read_u32::<LE>();
        }
        for _ in 0..hdr.get_num_a_descriptors() {
            // Skip 3 words
            cursor.read_u32::<LE>();
            cursor.read_u32::<LE>();
            cursor.read_u32::<LE>();
        }
        for _ in 0..hdr.get_num_b_descriptors() {
            // Skip 3 words
            cursor.read_u32::<LE>();
            cursor.read_u32::<LE>();
            cursor.read_u32::<LE>();
        }
        for _ in 0..hdr.get_num_w_descriptors() {
            // Skip 3 words
            cursor.read_u32::<LE>();
            cursor.read_u32::<LE>();
            cursor.read_u32::<LE>();
        }

        // Finally, read the raw section
        // TODO: Domain
        // Align to 16-byte boundary
        let before_pad = ((cursor.pos() + (16 - 1)) & !(16 - 1)) - cursor.pos();
        cursor.skip_read(before_pad);

        let input_objects = if this.domain_obj.is_some() {
            // Response have a "weird" domain header, at least in mephisto.
            //assert_eq!(domain_hdr.get_data_len() as usize, core::mem::size_of::<T>() + 8 + 8);
            // raw section size = Padding + domain header + SFCO/errcode + data size
            let input_objects = cursor.read_u32::<LE>() as usize;
            assert_eq!(hdr.get_raw_section_size() as u64, div_ceil((0x10 + 0x10 + 0x10 + core::mem::size_of::<T>() as usize + input_objects * 4) as u64, 4), "Invalid raw data size for domain");
            let _domain_id = cursor.read_u32::<LE>();
            cursor.skip_read(8);
            Some(input_objects)
        } else { None };

        // Find SFCO
        cursor.assert(b"SFCO\0\0\0\0");
        this.error = cursor.read_u64::<LE>();
        if this.error == 0 {
            if this.domain_obj.is_none() {
                assert_eq!(hdr.get_raw_section_size() as usize, (core::mem::size_of::<T>() + 8 + 8 + 0x10) / 4);
            }
            let raw = cursor.skip_read(core::mem::size_of::<T>());
            let raw = unsafe {
                (raw.as_ptr() as *const T).as_ref().unwrap()
            };
            this.ret = raw.clone();
        }

        if let Some(input_objects) = input_objects {
            for _ in 0..input_objects {
                this.objects.push(cursor.read_u32::<LE>());
            }
        }
        // Total padding should be 0x10
        cursor.skip_read(0x10 - before_pad);

        // TODO: Read the end

        if this.error != 0{
            Err(Error(this.error as u32))
        } else {
            Ok(this)
        }
    }

    pub fn get_raw(&self) -> &T {
        &self.ret
    }

    pub fn pop_handle(&mut self) -> KObject {
        self.handles.remove(0)
    }

    pub fn pop_domain_object(&mut self) -> Domain {
        Domain::new(self.domain_obj.clone().expect("Pop_domain_object called on a non-domain responses"), self.objects.remove(0))
    }
}
