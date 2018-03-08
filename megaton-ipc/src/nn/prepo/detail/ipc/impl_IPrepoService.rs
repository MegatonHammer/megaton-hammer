
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IPrepoService(Session);

impl IPrepoService {
	pub fn get_service() -> Result<IPrepoService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"prepo:a\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"prepo:m\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"prepo:u\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"prepo:s\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IPrepoService {
	// fn SaveReport(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SaveReportWithUser(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RequestImmediateTransmission(&self, ) -> Result<()> {
		let req = Request::new(10200)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetTransmissionStatus(&self, ) -> Result<i32> {
		let req = Request::new(10300)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn SaveSystemReport(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SaveSystemReportWithUser(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearStorage(&self, ) -> Result<()> {
		let req = Request::new(30100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsUserAgreementCheckEnabled(&self, ) -> Result<bool> {
		let req = Request::new(40100)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetUserAgreementCheckEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(40101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetStorageUsage(&self, ) -> Result<(i64, i64)> {
		let req = Request::new(90100)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: i64,
			unk1: i64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

}

impl FromKObject for IPrepoService {
	unsafe fn from_kobject(obj: KObject) -> IPrepoService {
		IPrepoService(Session::from_kobject(obj))
	}
}
