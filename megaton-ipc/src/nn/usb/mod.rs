pub mod hs;
pub mod pd;
pub mod ds;
pub mod pm;
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_endpoint_descriptor {
	bLength: u8,
	bDescriptorType: u8,
	bEndpointAddress: u8,
	bmAttributes: u8,
	wMaxPacketSize: u16,
	bInterval: u8,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_report_entry {
	urbId: u32,
	requestedSize: u32,
	transferredSize: u32,
	urbStatus: u32,
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_descriptor_data {
	idVendor: u16,
	idProduct: u16,
	bcdDevice: u16,
	manufacturer: [u8; 0x20],
	product: [u8; 0x20],
	serialNumber: [u8; 0x20],
}
#[repr(C)]
#[derive(Debug, Clone)]
pub struct usb_interface_descriptor {
	bLength: u8,
	bDescriptorType: u8,
	bInterfaceNumber: u8,
	bAlternateSetting: u8,
	bNumEndpoints: u8,
	bInterfaceClass: u8,
	bInterfaceSubClass: u8,
	bInterfaceProtocol: u8,
	iInterface: u8,
}
