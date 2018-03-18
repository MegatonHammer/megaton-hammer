#![feature(alloc, i128_type)]
#![no_std]
extern crate megaton_hammer;
extern crate spin;
extern crate alloc;
#[macro_use]
extern crate lazy_static;

pub mod nn;
pub mod nv;
pub mod nns;
pub type ServiceName = [u8; 8];
