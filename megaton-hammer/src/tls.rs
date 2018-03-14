//! Thread Local Storage
//!
//! On the switch, every thread is given a thread-specific IPC buffer, and a
//! pointer that the userland can use however they see fit. This is some kind of
//! global state, and as we all know, Rust hates global state. This module
//! provides a safe wrapper around the TLS structure.
//!

use core::ops::{Deref, DerefMut};
use core::fmt;

/// The basic unsafe building block of this module, and a very thin wrapper
/// around `mrs r0, tpidrro_el0`.
#[cfg(all(target_os = "switch", target_vendor = "roblabla", target_arch = "aarch64"))]
unsafe fn get_tls_space() -> *mut TlsStruct {
    let addr: *mut TlsStruct;
    asm!("mrs $0, tpidrro_el0" : "=r" (addr));
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
    ipc_buf: [0; 0x100],
    borrowed: false,
    unknown: [0; 0xF7],
    ctx: ::core::ptr::null_mut()
};

struct ThreadCtx;

/// The TLS buffer can be accessed through the tpidrro_el0 ARM system register.
/// It contains a 0x100 byte buffer used for IPC, a pointer to the userland
/// thread context, and some unused space.
#[repr(C)]
pub struct TlsStruct {
    pub ipc_buf: [u8; 0x100],
    borrowed: bool, // A bool is an i8. But it's not spec'd to be...
    unknown: [u8; 0xF7],
    ctx: *mut ThreadCtx
}
assert_eq_size!(tlsstruct_is_0x200_bytes; [u8; 0x200], TlsStruct);

impl fmt::Debug for TlsStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "TlsStruct {{ ipc_buf: ARRAY, borrowed: {:?}, unknown: padding, ctx: {:?} }}", self.borrowed, self.ctx)
    }
}

/// Wraps a borrowed reference to a TlsStruct. See the
/// [module-level documentation](index.html) for more.
pub struct TlsStructRefMut(*mut TlsStruct);

impl Drop for TlsStructRefMut {
    fn drop(&mut self) {
        unsafe {
            (*self.0).borrowed = false;
        }
    }
}

impl Deref for TlsStructRefMut {
    type Target = TlsStruct;
    fn deref(&self) -> &TlsStruct {
        unsafe { self.0.as_ref().expect("TlsStruct was null") }
    }
}

impl DerefMut for TlsStructRefMut {
    fn deref_mut(&mut self) -> &mut TlsStruct {
        unsafe { self.0.as_mut().expect("TlsStruct was null") }
    }
}

impl TlsStruct {
    /// 
    /// # Panics
    ///
    /// This will panic if another instance of TlsStructRefMut exists.
    pub fn borrow_mut() -> TlsStructRefMut {
        unsafe {
            let tls = get_tls_space();
            if (*tls).borrowed {
                panic!("Already mutably borrowed");
            } else {
                (*tls).borrowed = true;
                TlsStructRefMut(tls)
            }
        }
    }
}
