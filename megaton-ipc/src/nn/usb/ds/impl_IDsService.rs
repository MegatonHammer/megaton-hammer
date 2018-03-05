
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDsService(Session);

impl IDsService {
	pub fn BindDevice(&self, complexId: u32) -> Result<()> {
		let req = Request::new(0)
			.args(complexId)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn BindClientProcess(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetDsInterface(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetStateChangeEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetState(&self, ) -> Result<u32> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn SetVidPidBcd(&self, descriptor: &::nn::usb::usb_descriptor_data) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IDsService {
	unsafe fn from_kobject(obj: KObject) -> IDsService {
		IDsService(Session::from_kobject(obj))
	}
}
