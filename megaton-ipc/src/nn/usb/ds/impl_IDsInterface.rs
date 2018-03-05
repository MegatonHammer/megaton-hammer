
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDsInterface(Session);

impl IDsInterface {
	pub fn GetDsEndpoint(&self, unk0: &::nn::usb::usb_endpoint_descriptor) -> Result<::nn::usb::ds::IDsEndpoint> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	// fn GetSetupEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn EnableInterface(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DisableInterface(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CtrlInPostBufferAsync(&self, size: u32, buffer: u64) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req = Request::new(5)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn CtrlOutPostBufferAsync(&self, size: u32, buffer: u64) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req = Request::new(6)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetCtrlInCompletionEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetCtrlInReportData(&self, ) -> Result<[u8; 0x84]> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<[u8; 0x84]> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetCtrlOutCompletionEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetCtrlOutReportData(&self, ) -> Result<[u8; 0x84]> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<[u8; 0x84]> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn StallCtrl(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IDsInterface {
	unsafe fn from_kobject(obj: KObject) -> IDsInterface {
		IDsInterface(Session::from_kobject(obj))
	}
}
