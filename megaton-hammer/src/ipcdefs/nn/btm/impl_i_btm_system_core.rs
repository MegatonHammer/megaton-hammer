
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IBtmSystemCore<T>(T);

impl IBtmSystemCore<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IBtmSystemCore<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IBtmSystemCore(domain)),
			Err((sess, err)) => Err((IBtmSystemCore(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IBtmSystemCore<Session>> {
		Ok(IBtmSystemCore(self.0.duplicate()?))
	}
}

impl<T> Deref for IBtmSystemCore<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IBtmSystemCore<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IBtmSystemCore<T> {
	pub fn start_gamepad_pairing_impl(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_gamepad_pairing_impl(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_gamepad_pairing_database_impl(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_paired_gamepad_count_impl(&self, ) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn enable_radio_impl(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_radio_impl(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_radio_on_off_impl(&self, ) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn acquire_radio_event_impl(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn acquire_gamepad_pairing_event_impl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_gamepad_pairing_started_impl(&self, ) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IBtmSystemCore<T> {
	fn from(obj: T) -> IBtmSystemCore<T> {
		IBtmSystemCore(obj)
	}
}
