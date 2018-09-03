pub mod sf;
pub mod resolver;
pub type FdSet = u128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SockaddrIn {
	pub sin_len: u8,
	pub sin_family: u8,
	pub sin_port: u16,
	pub sin_addr: u32,
	pub sin_zero: [u8; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Timeout {
	pub sec: u64,
	pub usec: u64,
	pub off: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BsdBufferConfig {
	///  Observed 1 on 2.0 LibAppletWeb, 2 on 3.0
	pub version: u32,
	///  Size of the TCP transfer (send) buffer (initial or fixed).
	pub tcp_tx_buf_size: u32,
	///  Size of the TCP receive buffer (initial or fixed)
	pub tcp_rx_buf_size: u32,
	///  Maximum size of the TCP transfer (send) buffer. If it is 0, the size of
	///  the buffer is fixed to its initial value.
	pub tcp_tx_buf_max_size: u32,
	///  Maximum size of the TCP receive buffer. If it is 0, the size of the buffer
	///  is fixed to its initial value.
	pub tcp_rx_buf_max_size: u32,
	///  Size of the UDP transfer (send) buffer (typically 0x2400 bytes).
	pub udp_tx_buf_size: u32,
	///  Size of the UDP receive transfer buffer (typically 0xA500 bytes).
	pub udp_rx_buf_size: u32,
	///  Number of buffers for each socket (standard values range from 1 to 8)
	pub sb_efficiency: u32,
}
