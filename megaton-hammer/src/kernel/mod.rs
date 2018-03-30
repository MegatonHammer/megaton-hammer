pub mod svc;
pub mod session;

pub use self::session::*;
use error::{Error, Result};

#[derive(Debug)]
pub struct KObject(u32);

impl KObject {
    pub unsafe fn new(handle: u32) -> KObject {
        KObject(handle)
    }

    pub fn as_raw_handle(&self) -> u32 {
        self.0
    }
}

impl Drop for KObject {
    fn drop(&mut self) {
        let _ = unsafe { svc::close_handle(self.0) };
        // TODO: At least log failure to close handles
    }
}

pub trait FromKObject {
    // TODO: Is this unsafe ? I'm pretty sure it *isn't*, it'd just result in
    // predictable, non-ub crashes.
    unsafe fn from_kobject(obj: KObject) -> Self;
}

/// Transfer Memory Object
///
/// Used to transfer memory from one process to another. Several IPC APIs expect
/// to be initialized with some transfer memory. This API allows you to acquire
/// it.
///
/// Note: The memory acquired this way is lost *forever*. This is because we
/// cannot know when the other party is done with it.
#[derive(Debug)]
#[repr(transparent)]
pub struct TransferMemory(KObject);

impl TransferMemory {
    /// Allocates memory properly for transfer memory.
    pub fn new(size: usize) -> Result<TransferMemory> {
        use alloc::heap::{Alloc, Heap, Layout};
        use core::mem;

        // TODO: Use alloc_pages if present, default to normal allocator.
        let mem : *mut u8 = unsafe { Heap.alloc(Layout::from_size_align(size, 0x20000).unwrap()).unwrap() };
        // TODO: Allow passing some permission bits.
        let (res, out) = unsafe { svc::create_transfer_memory(mem as _, size as u64, 0) };

        // Leak the memory. We'll never be able to use it again.
        // TODO: *maybe* there's some kind of event I could listen to or
        // something, that would allow reclaiming the lost memory?
        mem::forget(mem);
        if res != 0 {
            return Err(Error(res));
        }
        Ok(TransferMemory(KObject(out)))
    }
}

impl AsRef<KObject> for TransferMemory {
    fn as_ref(&self) -> &KObject {
        &self.0
    }
}

// TODO: Impl from instead
impl Into<KObject> for TransferMemory {
    fn into(self) -> KObject {
        self.0
    }
}

impl FromKObject for TransferMemory {
    unsafe fn from_kobject(obj: KObject) -> TransferMemory {
        TransferMemory(obj)
    }
}
