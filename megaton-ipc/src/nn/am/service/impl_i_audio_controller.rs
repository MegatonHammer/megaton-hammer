
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAudioController<T>(T);

impl IAudioController<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAudioController<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAudioController(domain)),
			Err((sess, err)) => Err((IAudioController(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAudioController<Session>> {
		Ok(IAudioController(self.0.duplicate()?))
	}
}

impl<T> Deref for IAudioController<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAudioController<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAudioController<T> {
	pub fn set_expected_master_volume(&self, unk0: f32, unk1: f32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: f32,
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

	pub fn get_main_applet_expected_master_volume(&self, ) -> Result<f32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_library_applet_expected_master_volume(&self, ) -> Result<f32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn change_main_applet_master_volume(&self, unk0: f32, unk1: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: i64,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_transparent_volume_rate(&self, unk0: f32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAudioController<T> {
	fn from(obj: T) -> IAudioController<T> {
		IAudioController(obj)
	}
}
