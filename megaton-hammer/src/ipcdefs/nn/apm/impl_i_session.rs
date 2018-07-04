
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISession<T>(T);

impl ISession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISession(domain)),
			Err((sess, err)) => Err((ISession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISession<Session>> {
		Ok(ISession(self.0.duplicate()?))
	}
}

impl<T> Deref for ISession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISession<T> {
	pub fn set_performance_configuration(&self, unk0: ::ipcdefs::nn::apm::PerformanceMode, unk1: ::ipcdefs::nn::apm::PerformanceConfiguration) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::apm::PerformanceMode,
			unk1: ::ipcdefs::nn::apm::PerformanceConfiguration,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_performance_configuration(&self, unk0: ::ipcdefs::nn::apm::PerformanceMode) -> Result<::ipcdefs::nn::apm::PerformanceConfiguration> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::apm::PerformanceConfiguration> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for ISession<T> {
	fn from(obj: T) -> ISession<T> {
		ISession(obj)
	}
}
