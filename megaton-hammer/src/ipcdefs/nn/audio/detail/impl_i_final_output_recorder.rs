
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IFinalOutputRecorder<T>(T);

impl IFinalOutputRecorder<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IFinalOutputRecorder<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFinalOutputRecorder(domain)),
			Err((sess, err)) => Err((IFinalOutputRecorder(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFinalOutputRecorder<Session>> {
		Ok(IFinalOutputRecorder(self.0.duplicate()?))
	}
}

impl<T> Deref for IFinalOutputRecorder<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFinalOutputRecorder<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFinalOutputRecorder<T> {
	pub fn get_final_output_recorder_state(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start_final_output_recorder(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn stop_final_output_recorder(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn append_final_output_recorder_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn register_buffer_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_released_final_output_recorder_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn contains_final_output_recorder_buffer(&self, unk0: u64) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown7(&self, unk0: u64) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn append_final_output_recorder_buffer_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_released_final_output_recorder_buffer_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IFinalOutputRecorder<T> {
	fn from(obj: T) -> IFinalOutputRecorder<T> {
		IFinalOutputRecorder(obj)
	}
}
