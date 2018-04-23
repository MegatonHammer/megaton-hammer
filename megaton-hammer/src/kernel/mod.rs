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
        Ok(TransferMemory(KObject(out), Some(AllocatedMemory(mem, layout))))
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

/// Event Object
#[derive(Debug)]
#[repr(transparent)]
pub struct Event(KObject);

impl Event {
    pub fn wait(&self) -> Result<()> {
        let (res, _idx) = unsafe { svc::wait_synchronization(&(self.0).0, 1, !0) };
        if res == 0 {
            Ok(())
        } else {
            Err(Error(res))
        }
    }

    pub fn reset(&self) -> Result<()> {
        let res = unsafe { svc::reset_signal((self.0).0) };
        if res == 0 {
            Ok(())
        } else {
            Err(Error(res))
        }
    }
}

impl Into<KObject> for Event {
    fn into(self) -> KObject {
        self.0
    }
}

impl FromKObject for Event {
    unsafe fn from_kobject(obj: KObject) -> Event {
        Event(obj)
    }
}
