pub mod hs;
pub mod pd;
pub mod ds;
pub mod pm;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_endpoint_descriptor {
	pub bLength: u8,
	pub bDescriptorType: u8,
	pub bEndpointAddress: u8,
	pub bmAttributes: u8,
	pub wMaxPacketSize: u16,
	pub bInterval: u8,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_report_entry {
	pub urbId: u32,
	pub requestedSize: u32,
	pub transferredSize: u32,
	pub urbStatus: u32,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_descriptor_data {
	pub idVendor: u16,
	pub idProduct: u16,
	pub bcdDevice: u16,
	pub manufacturer: [u8; 0x20],
	pub product: [u8; 0x20],
	pub serialNumber: [u8; 0x20],
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_interface_descriptor {
	pub bLength: u8,
	pub bDescriptorType: u8,
	pub bInterfaceNumber: u8,
	pub bAlternateSetting: u8,
	pub bNumEndpoints: u8,
	pub bInterfaceClass: u8,
	pub bInterfaceSubClass: u8,
	pub bInterfaceProtocol: u8,
	pub iInterface: u8,
}
