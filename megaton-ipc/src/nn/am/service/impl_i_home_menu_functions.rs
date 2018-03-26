
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IHomeMenuFunctions<T>(T);

impl IHomeMenuFunctions<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IHomeMenuFunctions<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHomeMenuFunctions(domain)),
			Err((sess, err)) => Err((IHomeMenuFunctions(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHomeMenuFunctions<Session>> {
		Ok(IHomeMenuFunctions(self.0.duplicate()?))
	}
}

impl<T> Deref for IHomeMenuFunctions<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHomeMenuFunctions<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHomeMenuFunctions<T> {
	pub fn request_to_get_foreground(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn lock_foreground(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unlock_foreground(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop_from_general_channel(&self, ) -> Result<::nn::am::service::IStorage<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_pop_from_general_channel_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_home_button_writer_lock_accessor(&self, ) -> Result<::nn::am::service::ILockAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_writer_lock_accessor_ex(&self, unk0: i32) -> Result<::nn::am::service::ILockAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IHomeMenuFunctions<T> {
	fn from(obj: T) -> IHomeMenuFunctions<T> {
		IHomeMenuFunctions(obj)
	}
}
