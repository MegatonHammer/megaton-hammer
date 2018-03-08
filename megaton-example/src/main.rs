#![no_std]

extern crate megaton_crt0;
use core::fmt::Write;

fn main() {
    writeln!(megaton_crt0::LOG.lock(), "We are rust, and we are in the main!").unwrap();
}
