//! Thread Local Storage
//!
//! On the switch, every thread is given a thread-specific IPC buffer, and a
//! pointer that the userland can use however they see fit. This is some kind of
//! global state, and as we all know, Rust hates global state. This module
//! provides a safe wrapper around the TLS structure.
//!

use core::ops::{Deref, DerefMut};
use core::cell::UnsafeCell;
use core::fmt;
use alloc::BTreeMap;
use alloc::boxed::Box;

/// The basic unsafe building block of this module, and a very thin wrapper
/// around `mrs r0, tpidrro_el0`.
///
/// The result will always be a valid raw pointer.
#[cfg(all(target_os = "switch", target_vendor = "roblabla", target_arch = "aarch64"))]
unsafe fn get_tls_space() -> *mut TlsStruct {
    let addr: *mut TlsStruct;
    asm!("mrs $0, tpidrro_el0" : "=r" (addr));
    if addr.is_null() {
        panic!("TLS Pointer is null");
    }
    addr
}

#[cfg(not(all(target_os = "switch", target_vendor = "roblabla", target_arch = "aarch64")))]
unsafe fn get_tls_space() -> *mut TlsStruct {
    &mut TLS
}

// EWWWWWWW
#[cfg(not(all(target_os = "switch", target_vendor = "roblabla", target_arch = "aarch64")))]
unsafe impl Sync for TlsStruct {}

// This is a terrible thing.
#[cfg(not(all(target_os = "switch", target_vendor = "roblabla", target_arch = "aarch64")))]
static mut TLS: TlsStruct = TlsStruct {
    ipc_buf: ::core::cell::UnsafeCell::new([0; 0x100]),
    ipc_borrowed: false,
    unknown: [0; 0xF7],
    ctx: Box::new(ThreadCtx { locals: BTreeMap::new() })
};

#[derive(Debug)]
pub struct ThreadCtx {
    pub locals: BTreeMap<usize, *mut u8>
}

/// The TLS buffer can be accessed through the tpidrro_el0 ARM system register.
/// It contains a 0x100 byte buffer used for IPC, a pointer to the userland
/// thread context, and some unused space.
#[repr(C)]
pub struct TlsStruct {
    pub ipc_buf: UnsafeCell<[u8; 0x100]>,
    ipc_borrowed: bool, // A bool is an i8. But it's not spec'd to be...
    unknown: [u8; 0xF7],
    ctx: Box<ThreadCtx>
}

assert_eq_size!(tlsstruct_is_0x200_bytes; [u8; 0x200], TlsStruct);

impl fmt::Debug for TlsStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "TlsStruct {{ ipc_buf: ARRAY, ipc_borrowed: {:?}, unknown: padding, ctx: {:?} }}", self.ipc_borrowed, self.ctx)
    }
}

/// Wraps a borrowed reference to a TlsStruct. See the
/// [module-level documentation](index.html) for more.
pub struct IpcBufferRefMut(*mut TlsStruct);

impl Drop for IpcBufferRefMut {
    fn drop(&mut self) {
        unsafe {
            // This is safe because it's thread-local - only one thread can
            // access this variable.
            (*self.0).ipc_borrowed = false;
        }
    }
}

impl Deref for IpcBufferRefMut {
    type Target = [u8; 0x100];
    fn deref(&self) -> &[u8; 0x100] {
        unsafe { &*(*self.0).ipc_buf.get() }
    }
}

impl DerefMut for IpcBufferRefMut {
    fn deref_mut(&mut self) -> &mut [u8; 0x100] {
        unsafe { &mut *(*self.0).ipc_buf.get() }
    }
}

impl TlsStruct {
    /// FOR INTERNAL USE ONLY
    ///
    /// Sets up the ThreadCtx.
    pub unsafe fn init() {
        use core::{mem, ptr};
        let tls = get_tls_space();
        let new_ctx = Box::new(ThreadCtx { locals: BTreeMap::new() });
        let old_ctx = &mut (*tls).ctx;

        // Copy the newly allocated box in the old one without dropping what was
        // there before - most likely garbage!
        ptr::copy(&new_ctx, old_ctx, 1);
        mem::forget(new_ctx);
    }

    /// 
    /// # Panics
    ///
    /// This will panic if another instance of TlsStructRefMut exists.
    pub fn borrow_ipc_mut() -> IpcBufferRefMut {
        unsafe {
            let tls = get_tls_space();
            if (*tls).ipc_borrowed {
                panic!("Already mutably borrowed");
            } else {
                (*tls).ipc_borrowed = true;
                IpcBufferRefMut(tls)
            }
        }
    }

    pub fn get_thread_ctx() -> &'static mut ThreadCtx {
        unsafe {
            let tls = get_tls_space();
            // TODO: This is *wildly* unsafe. I can create multiple &mut things
            // :scream:
            //
            // I don't have a better solution for now.
            &mut (*tls).ctx
        }
    }
}
