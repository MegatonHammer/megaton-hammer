pub mod sf;
pub mod resolver;
pub type sockaddr = ();
pub type fd_set = u128;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct timeout {
	sec: u64,
	usec: u64,
	off: u64,
}
