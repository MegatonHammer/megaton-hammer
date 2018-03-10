//! # Megaton Hammer
//!
//! Welcome to the Fire Temple. I hope you're equipped with the Megaton Hammer.
//! Because it's time to hit some Rusty Switches! (I'm so sorry.)
//!
//! This crate's goal is to give the user all the low-level primitives needed
//! to interact with the Switch OS. It provides primitives for IPC, TLS,
//! syscalls.
//!
//! Note that this crate does not contain actual IPC definitions. This is left
//! for other crates (I'll be making one that uses SwIPC to create all the IPC).
//!
//! # Why another toolchain ?
//!
//! Because I firmly believe that as much stuff as possible should be written in
//! Rust. My first attempt at writing a rust toolchain reused libtransistor -
//! however, that proved to be more of a hassle than simply reimplementing
//! everything myself.
//!
//! Note that I plan on providing some form of compat layer with libtransistor
//! in the long term. (reimplementing its C API, but using rust). This should
//! allow porting libtransistor libraries to Megaton Hammer easily.
// TODO: I shouldn't need either of those, in an ideal world.
#![feature(asm, proc_macro, universal_impl_trait, cfg_target_vendor, global_asm)]

// Let's keep this no_std. Why ? Because it'll be used in std, that's why.
// Eventually, I might remove this.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std as core;

extern crate byteorder;

extern crate failure;
#[macro_use]
extern crate failure_derive;

extern crate bitfield_register;
extern crate bitfield_register_macro;

#[macro_use]
extern crate static_assertions;

extern crate arrayvec;

pub mod ipc;
pub mod tls;
pub mod kernel;
mod utils;

pub mod error {
    use core::fmt;

    #[derive(Clone, Copy, Fail)]
    pub struct Error(pub u32);

    impl fmt::Debug for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Error {:x} in module {}: {}", self.0, self.0 & (1 << 9) - 1, self.0 >> 9)
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // TODO: Actually print the module name, and description if we have it.
            write!(f, "Error in module {}: {}", self.0 & (1 << 9) - 1, self.0 >> 9)
        }
    }

    pub type Result<T> = ::core::result::Result<T, Error>;
}
