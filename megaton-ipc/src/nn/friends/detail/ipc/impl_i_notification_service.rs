
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INotificationService<T>(T);

impl INotificationService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INotificationService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INotificationService(domain)),
			Err((sess, err)) => Err((INotificationService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INotificationService<Session>> {
		Ok(INotificationService(self.0.duplicate()?))
	}
}

impl<T> Deref for INotificationService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INotificationService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INotificationService<T> {
	pub fn get_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn clear(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop(&self, ) -> Result<::nn::friends::detail::ipc::SizedNotificationInfo> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<::nn::friends::detail::ipc::SizedNotificationInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for INotificationService<T> {
	fn from(obj: T) -> INotificationService<T> {
		INotificationService(obj)
	}
}
