
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISession(Session);

impl AsRef<Session> for ISession {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISession {
	pub fn set_performance_configuration(&self, unk0: ::nn::apm::PerformanceMode, unk1: ::nn::apm::PerformanceConfiguration) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::apm::PerformanceMode,
			unk1: ::nn::apm::PerformanceConfiguration,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_performance_configuration(&self, unk0: ::nn::apm::PerformanceMode) -> Result<::nn::apm::PerformanceConfiguration> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<::nn::apm::PerformanceConfiguration> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for ISession {
	unsafe fn from_kobject(obj: KObject) -> ISession {
		ISession(Session::from_kobject(obj))
	}
}
