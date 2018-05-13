
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IApplicationAccessor<T>(T);

impl IApplicationAccessor<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IApplicationAccessor<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IApplicationAccessor(domain)),
			Err((sess, err)) => Err((IApplicationAccessor(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IApplicationAccessor<Session>> {
		Ok(IApplicationAccessor(self.0.duplicate()?))
	}
}

impl<T> Deref for IApplicationAccessor<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IApplicationAccessor<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IApplicationAccessor<T> {
	pub fn get_applet_state_changed_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn is_completed(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_exit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn terminate(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_for_application_to_get_foreground(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn terminate_all_library_applets(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(110)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn are_any_library_applets_left(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(111)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_library_applet(&self, ) -> Result<::nn::am::service::IAppletAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(112)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_application_id(&self, ) -> Result<::nn::ncm::ApplicationId> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(120)
			.args(())
			;
		let res : Response<::nn::ncm::ApplicationId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn push_launch_parameter(&self, unk0: u32, unk1: &::nn::am::service::IStorage<Session>) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(121)
			.args(unk0)
			.copy_handle(unk1.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_application_control_property(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_application_launch_property(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IApplicationAccessor<T> {
	fn from(obj: T) -> IApplicationAccessor<T> {
		IApplicationAccessor(obj)
	}
}
