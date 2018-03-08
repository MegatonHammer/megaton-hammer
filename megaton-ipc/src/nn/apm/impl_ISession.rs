
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISession(Session);

impl ISession {
	pub fn SetPerformanceConfiguration(&self, unk0: ::nn::apm::PerformanceMode, unk1: ::nn::apm::PerformanceConfiguration) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPerformanceConfiguration(&self, unk0: ::nn::apm::PerformanceMode) -> Result<::nn::apm::PerformanceConfiguration> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<::nn::apm::PerformanceConfiguration> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for ISession {
	unsafe fn from_kobject(obj: KObject) -> ISession {
		ISession(Session::from_kobject(obj))
	}
}
