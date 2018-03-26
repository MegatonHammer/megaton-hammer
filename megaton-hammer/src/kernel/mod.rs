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

#[derive(Debug)]
struct AllocatedMemory(*mut u8, ::alloc::heap::Layout);
impl Drop for AllocatedMemory {
    fn drop(&mut self) {
        use alloc::heap::{Heap, Alloc};
        unsafe { Heap.dealloc(self.0, self.1.clone()); }
    }
}


/// Transfer Memory Object
///
/// Used to transfer memory from one process to another. Several IPC APIs expect
/// to be initialized with some transfer memory. This API allows you to acquire
/// it.
#[derive(Debug)]
pub struct TransferMemory(KObject, Option<AllocatedMemory>);

impl TransferMemory {
    /// Allocates memory properly for transfer memory.
    pub fn new(size: usize) -> Result<TransferMemory> {
        use alloc::heap::{Heap, Layout, Alloc};
        use core::mem;

        // TODO: Use alloc_pages if present, default to normal allocator.
        // Align at the page level.
        let layout = Layout::from_size_align(size, 0x1000).expect("Given size is invalid");

        if layout.size() == 0 {
            panic!("Tried to allocate TransferMemory of size 0!");
        }

        let mem = match unsafe { Heap.alloc(layout.clone()) } {
            Ok(mem) => mem,
            Err(e) => Heap.oom(e)
        };

        let mut out = 0;
        // TODO: Allow passing some permission bits.
        let res = unsafe { svc::create_transfer_memory(&mut out, mem as _, size as u64, 0) };
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
        TransferMemory(obj, None)
    }
}
