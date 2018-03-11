
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IPrepoService(Session);

impl IPrepoService {
	pub fn new() -> Result<IPrepoService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"prepo:a\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"prepo:m\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"prepo:u\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.get_service(*b"prepo:s\0").map(|s| unsafe { IPrepoService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IPrepoService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPrepoService {
	// fn save_report(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn save_report_with_user(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_immediate_transmission(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10200)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_transmission_status(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10300)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn save_system_report(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn save_system_report_with_user(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_storage(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_user_agreement_check_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40100)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_user_agreement_check_enabled(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40101)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_storage_usage(&self, ) -> Result<(i64, i64)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(90100)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: i64,
			unk1: i64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

}

impl FromKObject for IPrepoService {
	unsafe fn from_kobject(obj: KObject) -> IPrepoService {
		IPrepoService(Session::from_kobject(obj))
	}
}
