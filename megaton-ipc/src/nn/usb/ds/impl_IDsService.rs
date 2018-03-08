
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IDsService(Session);

impl IDsService {
	pub fn get_service() -> Result<IDsService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"usb:ds\0\0").map(|s| unsafe { IDsService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IDsService {
	pub fn BindDevice(&self, complexId: u32) -> Result<()> {
		let req = Request::new(0)
			.args(complexId)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn BindClientProcess(&self, unk0: KObject) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetDsInterface(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetStateChangeEvent(&self, ) -> Result<KObject> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

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
