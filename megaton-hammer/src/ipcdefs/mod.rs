pub mod nn;
pub mod nv;
pub mod nns;
pub mod twili;
pub type ServiceName = [u8; 8];
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LoaderConfigEntryT {
	pub key: u32,
	pub flags: u32,
	pub value1: u64,
	pub value2: u64,
}
