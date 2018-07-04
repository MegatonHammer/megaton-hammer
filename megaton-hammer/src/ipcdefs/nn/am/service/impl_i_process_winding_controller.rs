
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IProcessWindingController<T>(T);

impl IProcessWindingController<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IProcessWindingController<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IProcessWindingController(domain)),
			Err((sess, err)) => Err((IProcessWindingController(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IProcessWindingController<Session>> {
		Ok(IProcessWindingController(self.0.duplicate()?))
	}
}

impl<T> Deref for IProcessWindingController<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IProcessWindingController<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IProcessWindingController<T> {
	pub fn get_launch_reason(&self, ) -> Result<::ipcdefs::nn::am::service::AppletProcessLaunchReason> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<::ipcdefs::nn::am::service::AppletProcessLaunchReason> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn open_calling_library_applet(&self, ) -> Result<::ipcdefs::nn::am::service::ILibraryAppletAccessor<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn push_context(&self, unk0: &::ipcdefs::nn::am::service::IStorage<Session>) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(21)
			.args(())
			.copy_handle(unk0.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop_context(&self, ) -> Result<::ipcdefs::nn::am::service::IStorage<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn cancel_winding_reservation(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn wind_and_do_reserved(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reserve_to_start_and_wait_and_unwind_this(&self, unk0: &::ipcdefs::nn::am::service::ILibraryAppletAccessor<Session>) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(40)
			.args(())
			.copy_handle(unk0.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IProcessWindingController<T> {
	fn from(obj: T) -> IProcessWindingController<T> {
		IProcessWindingController(obj)
	}
}
