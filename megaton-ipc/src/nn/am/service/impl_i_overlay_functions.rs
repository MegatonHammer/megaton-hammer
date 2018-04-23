
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IOverlayFunctions<T>(T);

impl IOverlayFunctions<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IOverlayFunctions<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IOverlayFunctions(domain)),
			Err((sess, err)) => Err((IOverlayFunctions(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IOverlayFunctions<Session>> {
		Ok(IOverlayFunctions(self.0.duplicate()?))
	}
}

impl<T> Deref for IOverlayFunctions<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IOverlayFunctions<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IOverlayFunctions<T> {
	pub fn begin_to_watch_short_home_button_message(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn end_to_watch_short_home_button_message(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_application_id_for_logo(&self, ) -> Result<::nn::ncm::ApplicationId> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::ncm::ApplicationId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_gpu_time_slice_boost(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_auto_sleep_time_and_dimming_time_enabled(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn terminate_application_and_set_reason(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_screen_shot_permission_globally(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IOverlayFunctions<T> {
	fn from(obj: T) -> IOverlayFunctions<T> {
		IOverlayFunctions(obj)
	}
}
