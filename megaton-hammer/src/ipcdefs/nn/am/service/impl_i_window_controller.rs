
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IWindowController<T>(T);

impl IWindowController<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IWindowController<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IWindowController(domain)),
			Err((sess, err)) => Err((IWindowController(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IWindowController<Session>> {
		Ok(IWindowController(self.0.duplicate()?))
	}
}

impl<T> Deref for IWindowController<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IWindowController<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IWindowController<T> {
	pub fn create_window(&self, unk0: ::ipcdefs::nn::am::service::WindowCreationOption) -> Result<::ipcdefs::nn::am::service::IWindow<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_applet_resource_user_id(&self, ) -> Result<::ipcdefs::nn::applet::AppletResourceUserId> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<::ipcdefs::nn::applet::AppletResourceUserId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn acquire_foreground_rights(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn release_foreground_rights(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reject_to_change_into_background(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IWindowController<T> {
	fn from(obj: T) -> IWindowController<T> {
		IWindowController(obj)
	}
}
