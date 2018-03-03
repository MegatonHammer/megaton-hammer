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
use bitfield_register_macro::register;
use byteorder::{LE};
use kernel::KObject;

use utils::{CursorWrite, CursorRead};
use error::*;

// Fits in a QWORD
// TODO: Migrate bitfield_register_macro to a custom Derive
/// Represents the header of a packed IPC message, to be sent to the kernel for
/// processing. This is usually written in the first 0x100 byes of TLS, but
/// any memory locaction may be used, if used with `svcSendSyncRequestWithUserBuffer`.
#[register()]
struct MsgPackedHdr {
    #[bitfield(from=0, to=15)]
    ty: u16,
    #[bitfield(from=16, to=19)]
    num_x_descriptors: u8,
    #[bitfield(from=20, to=23)]
    num_a_descriptors: u8,
    #[bitfield(from=24, to=27)]
    num_b_descriptors: u8,
    #[bitfield(from=28, to=31)]
    num_w_descriptors: u8,
    #[bitfield(from=32, to=41)]
    raw_section_size: u16,
    #[bitfield(from=42, to=45)]
    c_descriptor_flags: u8,
    #[bitfield(from=52, to=62)]
    unknown: u16,
    #[bitfield(at=63)]
    enable_handle_descriptor: bool
}

#[register()]
struct HandleDescriptorHeader {
    #[bitfield(at = 0)]
    send_pid: bool,
    #[bitfield(from=1, to=4)]
    num_copy_handles: u8,
    #[bitfield(from=5, to=8)]
    num_move_handles: u8,
}

#[derive(Debug)]
pub enum MessageType {
    Request,
    Control,
    Unknown(u16)
}

#[derive(Debug)]
pub struct IPCBuffer<'a> {
    // Address to the value
    val: *mut (),
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
    fn from_ptr<T>(val: &'a T, ty: u64) -> IPCBuffer {
        IPCBuffer {
            val: val as *const _ as *mut _,
            size: core::mem::size_of::<T>(),
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
pub struct Request<'a, 'b, 'c, RAW> {
    ty: u16,
    send_pid: bool,
    buffers: Option<&'a [IPCBuffer<'a>]>, // Also has a max of 16 per type.
    copy_handles: Option<&'b [KObject]>, // Max 16. Maybe this should just be a vec
    move_handles: Option<&'c [KObject]>, // Max 16

    // The data section is built in !
    cmd_id: u64,
    // TODO: I really feel like this *ought* to be 
    args: Option<RAW>,
}

// TODO: Figure out a way to avoid T: Clone ?
// TODO: Maybe I should just store a *pointer* to T ?
impl<'a, 'b, 'c, T: Clone> Request<'a, 'b, 'c, T> {
    pub fn new(id: u64) -> Self {
        Request {
            ty: 4,
            cmd_id: id,
            send_pid: false,
            buffers: None,
            copy_handles: None,
            move_handles: None,
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

    // TODO: Take ref to raw ?
    pub fn args(mut self, args: T) -> Self {
        self.args = Some(args);
        self
    }

    // Write the data to an IPC buffer to be sent to the Switch OS.
    pub fn pack(&self, data: &mut [u8]) {
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
            hdr.set_num_x_descriptors(0);
            hdr.set_num_a_descriptors(0);
            hdr.set_num_b_descriptors(0);
            hdr.set_num_w_descriptors(0);
            hdr.set_c_descriptor_flags(0);
            // TODO: Domain, type 0xA. 0x10 = padding, 8 = sfci, 8 = cmdid.
            hdr.set_raw_section_size(((0x10 + 8 + 8 + core::mem::size_of::<T>()) / 4) as u16);
            if self.copy_handles.map(|x| x.len()).unwrap_or(0) > 0 ||
                self.move_handles.map(|x| x.len()).unwrap_or(0) > 0 || self.send_pid {

                hdr.set_enable_handle_descriptor(true);
            }
        }

        let mut _x_descriptors = 0;
        let mut _a_descriptors = 0;
        let mut _b_descriptors = 0;
        let mut _w_descriptors = 0;

        // First, write the handle descriptor
        if self.copy_handles.map(|x| x.len()).unwrap_or(0) > 0 ||
            self.move_handles.map(|x| x.len()).unwrap_or(0) > 0 || self.send_pid {

            // Handle Descriptor Header
            {
                let descriptor_hdr = cursor.skip_write(core::mem::size_of::<HandleDescriptorHeader>());
                let descriptor_hdr = unsafe {
                    (descriptor_hdr.as_mut_ptr() as *mut HandleDescriptorHeader).as_mut().unwrap()
                };

                // Write the header
                descriptor_hdr.set_num_copy_handles(self.copy_handles.map(|x| x.len()).unwrap_or(0) as u8);
                descriptor_hdr.set_num_move_handles(self.move_handles.map(|x| x.len()).unwrap_or(0) as u8);
                descriptor_hdr.set_send_pid(self.send_pid);
            }

            // Seek 8 if we have to send pid. We don't actually write the pid.
            if self.send_pid {
                cursor.skip_write(8);
            }

            // Write copy and move handles
            for hnd in self.copy_handles.unwrap_or(&[]) {
                cursor.write_u32::<LE>((*hnd).0);
            }
            for hnd in self.move_handles.unwrap_or(&[]) {
                cursor.write_u32::<LE>((*hnd).0);
            }
        }

        // TODO: Write buffers
        //for buf in 
        // Write raw data section. Save current position to find out the size of
        // the raw section later.

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

    #[cfg(feature = "core")]
    pub fn show_packed(self) -> Self {
        let mut arr = [0; 0x100];
        self.pack(&mut arr);
        // Let's emulate what pegaswitch seems to be doing. They split the chunk
        // in lines of 16 bytes
        for (i, chunk) in arr.chunks(16).enumerate() {
            // Print the current offset (do some padding if necessary so it all
            // aligns correctly).
            print!("ipcm+{:#01$x} |", i * 16, f64::log2(arr.len() as f64).ceil() as usize - f64::log2(((i + 1) * 16) as f64).ceil() as usize);

            // Print the bytes one by one. Put an extra space in the middle
            for (i, b) in chunk.iter().enumerate() {
                if i == 8 {
                    print!(" ");
                }
                print!(" {:02x}", b);
            }
            // Fill missing with spaces.
            print!("{}", "   ".repeat(16 - chunk.len()));

            // And now show the ASCII representation. Replace unprintable
            // characters with  a '.'
            print!(" | ");
            for b in chunk {
                print!("{}", if (*b as char).is_ascii_graphic() { *b as char } else { '.' });
            }
            println!(" |");
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
        assert_eq!(hdr.get_ty(), 4);
        assert_eq!(hdr.get_raw_section_size() as usize, core::mem::size_of::<T>());

        // First, read the handle descriptor
        if hdr.get_enable_handle_descriptor() {
            let descriptor_hdr = cursor.skip_read(core::mem::size_of::<HandleDescriptorHeader>());
            let descriptor_hdr = unsafe {
                (descriptor_hdr.as_ptr() as *const HandleDescriptorHeader).as_ref().unwrap()
            };

            if descriptor_hdr.get_send_pid() {
                // TODO
                let _pid = cursor.read_u64::<LE>();
            }
            for _ in 0..descriptor_hdr.get_num_copy_handles() {
                this.handles.push(KObject(cursor.read_u32::<LE>()));
            }
            for _ in 0..descriptor_hdr.get_num_move_handles() {
                this.handles.push(KObject(cursor.read_u32::<LE>()));
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
        cursor.assert(b"SFCO\0\0\0\0");
        this.error = cursor.read_u64::<LE>();
        let raw = cursor.skip_read(core::mem::size_of::<T>());
        let raw = unsafe {
            (raw.as_ptr() as *const T).as_ref().unwrap()
        };
        this.ret = raw.clone();
        Ok(this)
    }

    pub fn get_raw(&self) -> &T {
        &self.ret
    }
}
