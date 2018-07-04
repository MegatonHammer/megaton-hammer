
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IHardwareOpusDecoder<T>(T);

impl IHardwareOpusDecoder<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IHardwareOpusDecoder<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHardwareOpusDecoder(domain)),
			Err((sess, err)) => Err((IHardwareOpusDecoder(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHardwareOpusDecoder<Session>> {
		Ok(IHardwareOpusDecoder(self.0.duplicate()?))
	}
}

impl<T> Deref for IHardwareOpusDecoder<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHardwareOpusDecoder<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHardwareOpusDecoder<T> {
	// fn decode_interleaved(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_context(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn decode_interleaved_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_context_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown4(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown5(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IHardwareOpusDecoder<T> {
	fn from(obj: T) -> IHardwareOpusDecoder<T> {
		IHardwareOpusDecoder(obj)
	}
}
