
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAudioDevice<T>(T);

impl IAudioDevice<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAudioDevice<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAudioDevice(domain)),
			Err((sess, err)) => Err((IAudioDevice(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAudioDevice<Session>> {
		Ok(IAudioDevice(self.0.duplicate()?))
	}
}

impl<T> Deref for IAudioDevice<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAudioDevice<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAudioDevice<T> {
	// fn list_audio_device_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_audio_device_output_volume(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_audio_device_output_volume(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_active_audio_device_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn query_audio_device_system_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_active_channel_count(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn list_audio_device_name_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_audio_device_output_volume_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_audio_device_output_volume_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_active_audio_device_name_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn query_audio_device_input_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn query_audio_device_output_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IAudioDevice<T> {
	fn from(obj: T) -> IAudioDevice<T> {
		IAudioDevice(obj)
	}
}
