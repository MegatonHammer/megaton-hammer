
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ILibraryAppletAccessor<T>(T);

impl ILibraryAppletAccessor<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ILibraryAppletAccessor<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILibraryAppletAccessor(domain)),
			Err((sess, err)) => Err((ILibraryAppletAccessor(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILibraryAppletAccessor<Session>> {
		Ok(ILibraryAppletAccessor(self.0.duplicate()?))
	}
}

impl<T> Deref for ILibraryAppletAccessor<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILibraryAppletAccessor<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILibraryAppletAccessor<T> {
	pub fn get_applet_state_changed_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn is_completed(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_exit(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn terminate(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_result(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_out_of_focus_application_suspending_enabled(&self, unk0: bool) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(50)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn push_in_data(&self, unk0: &::ipcdefs::nn::am::service::IStorage<Session>) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(100)
			.args(())
			.copy_handle(unk0.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop_out_data(&self, ) -> Result<::ipcdefs::nn::am::service::IStorage<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn push_extra_storage(&self, unk0: &::ipcdefs::nn::am::service::IStorage<Session>) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(102)
			.args(())
			.copy_handle(unk0.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn push_interactive_in_data(&self, unk0: &::ipcdefs::nn::am::service::IStorage<Session>) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(103)
			.args(())
			.copy_handle(unk0.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop_interactive_out_data(&self, ) -> Result<::ipcdefs::nn::am::service::IStorage<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(104)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_pop_out_data_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(105)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_pop_interactive_out_data_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(106)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn needs_to_exit_process(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(110)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_library_applet_info(&self, ) -> Result<::ipcdefs::nn::am::service::LibraryAppletInfo> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(120)
			.args(())
			;
		let res : Response<::ipcdefs::nn::am::service::LibraryAppletInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn request_for_applet_to_get_foreground(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(150)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_indirect_layer_consumer_handle(&self, unk0: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(160)
			.args(unk0)
			.send_pid()
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for ILibraryAppletAccessor<T> {
	fn from(obj: T) -> ILibraryAppletAccessor<T> {
		ILibraryAppletAccessor(obj)
	}
}
