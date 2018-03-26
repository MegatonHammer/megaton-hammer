//! Core IPC primitives
//!
//! This module contains the low-level primitives used for IPC.
//!

// TODO: Separate HIPC from CMIF ? HIPC is the low-level IPC mechanism, whereas
// CMIF is the higher-level Command-based IPC protocol.

use core;
use core::mem;
use core::marker::PhantomData;
use arrayvec::ArrayVec;
use byteorder::{LE};
use kernel::KObject;
use bit_field::BitField;

use utils::{CursorWrite, CursorRead};
use error::*;

// Fits in a QWORD
// TODO: Migrate bitfield_register_macro to a custom Derive
/// Represents the header of a packed IPC message, to be sent to the kernel for
/// processing. This is usually written in the first 0x100 byes of TLS, but
/// any memory locaction may be used, if used with `svcSendSyncRequestWithUserBuffer`.
#[repr(transparent)]
struct MsgPackedHdr(u64);

impl MsgPackedHdr {
    pub fn get_ty(&self) -> u16 {
        self.0.get_bits(0..16) as u16
    }

    pub fn set_ty(&mut self, n: u16) {
        self.0 = *self.0.set_bits(0..16, n as u64)
    }

    pub fn get_num_x_descriptors(&self) -> u8 {
        self.0.get_bits(16..20) as u8
    }

    pub fn set_num_x_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(16..20, n as u64)
    }

    pub fn get_num_a_descriptors(&self) -> u8 {
        self.0.get_bits(20..24) as u8
    }

    pub fn set_num_a_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(20..24, n as u64)
    }

    pub fn get_num_b_descriptors(&self) -> u8 {
        self.0.get_bits(24..28) as u8
    }

    pub fn set_num_b_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(24..28, n as u64)
    }

    pub fn get_num_w_descriptors(&self) -> u8 {
        self.0.get_bits(28..32) as u8
    }

    pub fn set_num_w_descriptors(&mut self, n: u8) {
        self.0 = *self.0.set_bits(28..32, n as u64)
    }

    pub fn get_raw_section_size(&self) -> u16 {
        self.0.get_bits(32..42) as u16
    }

    pub fn set_raw_section_size(&mut self, n: u16) {
        self.0 = *self.0.set_bits(32..42, n as u64)
    }

    pub fn get_c_descriptor_flags(&self) -> u8 {
        self.0.get_bits(42..46) as u8
    }

    pub fn set_c_descriptor_flags(&mut self, n: u8) {
        self.0 = *self.0.set_bits(42..46, n as u64)
    }

    //pub fn get_unknown(&self) -> u16 {
    //    self.0.get_bits(52..63) as u16
    //}

    pub fn get_enable_handle_descriptor(&self) -> bool {
        self.0.get_bit(63)
    }

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

assert_eq_size!(AssertHandleDescriptorHeader; u16, HandleDescriptorHeader);

#[derive(Debug)]
pub enum MessageType {
    Request,
    Control,
    Unknown(u16)
}

#[derive(Debug)]
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
            addr: val.as_ptr() as usize,
            size: core::mem::size_of::<T>() * val.len(),
            ty,
            phantom: PhantomData
        }
    }
    pub fn from_mut_slice<T>(val: &'a mut [T], ty: u64) -> IPCBuffer {
        // TODO: Verify type and val mutability
        IPCBuffer {
            addr: val.as_ptr() as usize,
            size: core::mem::size_of::<T>() * val.len(),
            ty,
            phantom: PhantomData
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

#[derive(Debug)]
pub struct Request<'a, 'b, RAW> {
    ty: u16,
    send_pid: bool,
    x_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    a_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    b_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    c_descriptors: ArrayVec<[IPCBuffer<'a>; 16]>,
    copy_handles: ArrayVec<[&'b KObject; 16]>,
    move_handles: ArrayVec<[KObject; 16]>,

    // The data section is built in !
    cmd_id: u64,
    // TODO: I really feel like this *ought* to be 
    args: Option<RAW>,
}

// TODO: Figure out a way to avoid T: Clone ?
// TODO: Maybe I should just store a *pointer* to T ?
impl<'a, 'b, T: Clone> Request<'a, 'b, T> {
    pub fn new(id: u64) -> Self {
        Request {
            ty: 4,
            cmd_id: id,
            send_pid: false,
            x_descriptors: ArrayVec::new(),
            a_descriptors: ArrayVec::new(),
            b_descriptors: ArrayVec::new(),
            c_descriptors: ArrayVec::new(),
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
        enum Direction { In, Out }
        enum Family { AB, XC }

        if buf.ty & 0x20 == 0 {
            let direction = if buf.ty & 0b0001 != 0 { Direction::In }
            else if buf.ty & 0b0010 != 0 { Direction::Out }
            else { panic!("Invalid buffer type {}", buf.ty); };

            let family = if buf.ty & 0b0100 != 0 { Family::AB }
            else if buf.ty & 0b1000 != 0 { Family::XC }
            else { panic!("Invalid buffer type {}", buf.ty); };

            match (direction, family) {
                (Direction::In, Family::AB) => self.a_descriptors.push(buf),
                (Direction::Out, Family::AB) => self.b_descriptors.push(buf),
                (Direction::In, Family::XC) => self.x_descriptors.push(buf),
                (Direction::Out, Family::XC) => ()
            }
        } else if buf.ty == 0x21 {
            self.a_descriptors.push(buf);
            self.x_descriptors.push(IPCBuffer::null());
        } else if buf.ty == 0x22 {
            self.b_descriptors.push(buf);
            self.c_descriptors.push(IPCBuffer::null());
        } else {
            panic!("Invalid descriptor type: {}", buf.ty);
        }
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

    // Write the data to an IPC buffer to be sent to the Switch OS.
    // TODO: If this is not sent, it can leak move handles!
    pub fn pack(self, data: &mut [u8]) {
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
            hdr.set_num_x_descriptors(self.x_descriptors.len() as u8);
            hdr.set_num_a_descriptors(self.a_descriptors.len() as u8);
            hdr.set_num_b_descriptors(self.b_descriptors.len() as u8);
            hdr.set_num_w_descriptors(0);
            if self.c_descriptors.len() == 0 {
                hdr.set_c_descriptor_flags(0);
            } else if self.c_descriptors.len() == 1 {
                hdr.set_c_descriptor_flags(2);
            } else {
                hdr.set_c_descriptor_flags(2 + self.c_descriptors.len() as u8);
            }

            // TODO: Domain, type 0xA. 0x10 = padding, 8 = sfci, 8 = cmdid.
            hdr.set_raw_section_size(((0x10 + 8 + 8 + core::mem::size_of::<T>()) / 4) as u16);
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

        for (i, buf) in self.x_descriptors.iter().enumerate() {
            assert!(buf.addr >> 39 == 0, "Invalid buffer address");
            assert!(buf.size >> 16 == 0, "Invalid buffer size");
            let num = i & 0b111000111111
                | ((buf.addr >> 36) & 0b111) << 6
                | ((buf.addr >> 32) & 0b1111) << 12
                | buf.size << 16;
            cursor.write_u32::<LE>(num as u32);
            cursor.write_u32::<LE>((buf.addr & 0xFFFFFFFF) as u32)
        }

        for buf in self.a_descriptors.iter().chain(self.b_descriptors.iter()) {
            assert!(buf.addr >> 39 == 0, "Invalid buffer address");
            assert!(buf.size >> 35 == 0, "Invalid buffer size");
            assert!(buf.ty >> 8 == 0, "Invalid descriptor permission");

            cursor.write_u32::<LE>((buf.size & 0xFFFFFFFF) as u32);
            cursor.write_u32::<LE>((buf.addr & 0xFFFFFFFF) as u32);

            let num = buf.ty as usize >> 6 // flags
                | ((buf.addr >> 36) & 0b111) << 2
                | ((buf.size >> 32) & 0b1111) << 24
                | ((buf.addr >> 32) & 0b1111) << 28;
            cursor.write_u32::<LE>(num as u32);
        }

        // TODO: W descriptors would go there.

        // Align to 16-byte boundary
        let before_pad = ((cursor.pos() + (16 - 1)) & !(16 - 1)) - cursor.pos();
        cursor.skip_write(before_pad);
        // TODO: Domain messages
        cursor.write(b"SFCI\0\0\0\0");
        cursor.write_u64::<LE>(self.cmd_id);
        // TODO: Should blow up if that's not true. Alternatively: This should
        // not even be possible from an API perspective ?
        if let Some(ref args) = self.args {
            let raw_data = cursor.skip_write(core::mem::size_of::<T>());
            let raw_data = unsafe {
                (raw_data.as_mut_ptr() as *mut T).as_mut().unwrap()
            };
            *raw_data = args.clone();
        }

        // Total padding should be 0x10
        cursor.skip_write(0x10 - before_pad);
        // TODO: Buffer 0xA.


        // TODO: Write c buffers
    }

    pub fn show_packed(self, f: &mut core::fmt::Write) -> Self {
        // Let's make a copy. **WE NEED TO FORGET IT**
        // TODO: Maybe there's a cleaner way to unsafely make a copy without
        // transmuting ?
        let other_self : Self = unsafe { core::mem::transmute_copy(&self) };
        let mut arr = [0; 0x100];
        other_self.pack(&mut arr);

        // Let's emulate what xxd is doing, so we can turn it back to binary
        // with xxd -r
        for (i, chunk) in arr.chunks(16).enumerate() {
            // Print the current offset (do some padding if necessary so it all
            // aligns correctly).
            let log2 = 64 - arr.len().leading_zeros();
            let log2 = if log2 % 4 == 0 { log2 / 4 } else { (log2 / 4) + 1 };
            let _ = write!(f, "{:01$x}:", i * 16, log2 as usize);

            // Print the bytes one by one. Put an extra space in the middle
            for (i, b) in chunk.iter().enumerate() {
                if i % 2 == 0 {
                    let _ = write!(f, " ");
                }
                let _ = write!(f, "{:02x}", b);
            }
            // Fill missing with spaces.
            for _ in 0..16 - chunk.len() {
                let _ = write!(f, "{}", "   ");
            }

            // And now show the ASCII representation. Replace unprintable
            // characters with  a '.'
            let _ = write!(f, "  ");
            for b in chunk {
                let _ = write!(f, "{}", if (*b as char).is_ascii_graphic() { *b as char } else { '.' });
            }
            let _ = writeln!(f, "");
        }
        self
    }
}

// TODO: I could make RAW be a reference, if I'm OK with tying it to the TLS
// buffer's lifetime (somehow).
#[derive(Debug)]
pub struct Response<RAW> {
    error: u64,
    pid: Option<u64>,
    handles: ArrayVec<[KObject; 32]>,
    ret: RAW
}

impl<T: Clone> Response<T> {
    // TODO: Mark unpack as unsafe (for all the obvious reasons)
    pub fn unpack(data: &[u8]) -> Result<Response<T>> {
        let mut this : Response<T> = Response {
            error: 0,
            pid: None,
            handles: ArrayVec::new(),
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
        assert_eq!(hdr.get_ty(), 0);
        assert_eq!(hdr.get_raw_section_size() as usize, (core::mem::size_of::<T>() + 8 + 8 + 0x10) / 4);

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
        // Find SFCO
        cursor.assert(b"SFCO\0\0\0\0");
        this.error = cursor.read_u64::<LE>();
        let raw = cursor.skip_read(core::mem::size_of::<T>());
        let raw = unsafe {
            (raw.as_ptr() as *const T).as_ref().unwrap()
        };
        this.ret = raw.clone();
        // Total padding should be 0x10
        cursor.skip_read(0x10 - before_pad);

        // TODO: Read the end

        Ok(this)
    }

    pub fn get_raw(&self) -> &T {
        &self.ret
    }

    pub fn pop_handle(&mut self) -> KObject {
        self.handles.remove(0)
    }
}
