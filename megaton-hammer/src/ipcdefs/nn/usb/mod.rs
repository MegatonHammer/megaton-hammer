pub mod hs;
pub mod pd;
pub mod ds;
pub mod pm;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UsbEndpointDescriptor {
	pub b_length: u8,
	pub b_descriptor_type: u8,
	pub b_endpoint_address: u8,
	pub bm_attributes: u8,
	pub w_max_packet_size: u16,
	pub b_interval: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UsbReportEntry {
	pub urb_id: u32,
	pub requested_size: u32,
	pub transferred_size: u32,
	pub urb_status: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UsbDescriptorData {
	pub id_vendor: u16,
	pub id_product: u16,
	pub bcd_device: u16,
	pub manufacturer: [u8; 0x20],
	pub product: [u8; 0x20],
	pub serial_number: [u8; 0x20],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UsbInterfaceDescriptor {
	pub b_length: u8,
	pub b_descriptor_type: u8,
	pub b_interface_number: u8,
	pub b_alternate_setting: u8,
	pub b_num_endpoints: u8,
	pub b_interface_class: u8,
	pub b_interface_sub_class: u8,
	pub b_interface_protocol: u8,
	pub i_interface: u8,
}
