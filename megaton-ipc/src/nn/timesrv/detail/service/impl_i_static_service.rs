
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IStaticService(Session);

impl IStaticService {
	pub fn new() -> Result<Arc<IStaticService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IStaticService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"time:s\0\0").map(|s| Arc::new(unsafe { IStaticService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		let r = sm.get_service(*b"time:u\0\0").map(|s| Arc::new(unsafe { IStaticService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		let r = sm.get_service(*b"time:r\0\0").map(|s| Arc::new(unsafe { IStaticService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}

		let r = sm.get_service(*b"time:a\0\0").map(|s| Arc::new(unsafe { IStaticService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IStaticService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IStaticService {
	pub fn get_standard_user_system_clock(&self, ) -> Result<::nn::timesrv::detail::service::ISystemClock> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_standard_network_system_clock(&self, ) -> Result<::nn::timesrv::detail::service::ISystemClock> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_standard_steady_clock(&self, ) -> Result<::nn::timesrv::detail::service::ISteadyClock> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_time_zone_service(&self, ) -> Result<::nn::timesrv::detail::service::ITimeZoneService> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_standard_local_system_clock(&self, ) -> Result<::nn::timesrv::detail::service::ISystemClock> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn is_standard_user_system_clock_automatic_correction_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_standard_user_system_clock_automatic_correction_enabled(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_standard_network_system_clock_accuracy_sufficient(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IStaticService {
	unsafe fn from_kobject(obj: KObject) -> IStaticService {
		IStaticService(Session::from_kobject(obj))
	}
}
