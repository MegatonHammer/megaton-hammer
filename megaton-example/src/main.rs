#![no_std]

extern crate megaton_crt0;
use core::fmt::Write;

fn main() {
    unsafe {
        writeln!(megaton_crt0::LOG, "We are in the main!").unwrap();
    };
}
