pub mod svc;
pub mod session;

pub use self::session::*;

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
