
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IApplicationCreator<T>(T);

impl IApplicationCreator<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IApplicationCreator<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IApplicationCreator(domain)),
			Err((sess, err)) => Err((IApplicationCreator(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IApplicationCreator<Session>> {
		Ok(IApplicationCreator(self.0.duplicate()?))
	}
}

impl<T> Deref for IApplicationCreator<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IApplicationCreator<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IApplicationCreator<T> {
	pub fn create_application(&self, unk0: ::nn::ncm::ApplicationId) -> Result<::nn::am::service::IApplicationAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn pop_launch_requested_application(&self, ) -> Result<::nn::am::service::IApplicationAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn create_system_application(&self, unk0: ::nn::ncm::SystemApplicationId) -> Result<::nn::am::service::IApplicationAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn pop_floating_application_for_development(&self, ) -> Result<::nn::am::service::IApplicationAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IApplicationCreator<T> {
	fn from(obj: T) -> IApplicationCreator<T> {
		IApplicationCreator(obj)
	}
}
