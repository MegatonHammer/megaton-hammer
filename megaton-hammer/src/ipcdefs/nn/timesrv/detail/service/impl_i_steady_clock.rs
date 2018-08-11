
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISteadyClock<T>(T);

impl ISteadyClock<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISteadyClock<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISteadyClock(domain)),
			Err((sess, err)) => Err((ISteadyClock(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISteadyClock<Session>> {
		Ok(ISteadyClock(self.0.duplicate()?))
	}
}

impl<T> Deref for ISteadyClock<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISteadyClock<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISteadyClock<T> {
	pub fn get_current_time_point(&self, ) -> Result<::ipcdefs::nn::time::SteadyClockTimePoint> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<::ipcdefs::nn::time::SteadyClockTimePoint> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_test_offset(&self, ) -> Result<::ipcdefs::nn::TimeSpanType> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<::ipcdefs::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_test_offset(&self, unk0: ::ipcdefs::nn::TimeSpanType) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_rtc_value(&self, ) -> Result<i64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn is_rtc_reset_detected(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_setup_result_value(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(102)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_internal_offset(&self, ) -> Result<::ipcdefs::nn::TimeSpanType> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(200)
			.args(())
			;
		let res : Response<::ipcdefs::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(all(feature = "switch-3.0.0", not(feature = "switch-4.0.0")))]
	pub fn set_internal_offset(&self, unk0: ::ipcdefs::nn::TimeSpanType) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(201)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISteadyClock<T> {
	fn from(obj: T) -> ISteadyClock<T> {
		ISteadyClock(obj)
	}
}
