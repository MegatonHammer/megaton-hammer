
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDisplayController<T>(T);

impl IDisplayController<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDisplayController<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDisplayController(domain)),
			Err((sess, err)) => Err((IDisplayController(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDisplayController<Session>> {
		Ok(IDisplayController(self.0.duplicate()?))
	}
}

impl<T> Deref for IDisplayController<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDisplayController<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDisplayController<T> {
	// fn get_last_foreground_capture_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn update_last_foreground_capture_image(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_last_application_capture_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_caller_applet_capture_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn update_caller_applet_capture_image(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_last_foreground_capture_image_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_last_application_capture_image_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_caller_applet_capture_image_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn take_screen_shot_of_own_layer(&self, unk0: bool, unk1: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_last_application_capture_buffer(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn release_last_application_capture_buffer(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_last_foreground_capture_buffer(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn release_last_foreground_capture_buffer(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_caller_applet_capture_buffer(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn release_caller_applet_capture_buffer(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn acquire_last_application_capture_buffer_ex(&self, ) -> Result<(bool, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn acquire_last_foreground_capture_buffer_ex(&self, ) -> Result<(bool, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(17)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn acquire_caller_applet_capture_buffer_ex(&self, ) -> Result<(bool, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(18)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn clear_capture_buffer(&self, unk0: bool, unk1: i32, unk2: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
			unk2: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_applet_transition_buffer(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(21)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDisplayController<T> {
	fn from(obj: T) -> IDisplayController<T> {
		IDisplayController(obj)
	}
}
