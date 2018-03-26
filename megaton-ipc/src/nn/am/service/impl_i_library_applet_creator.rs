
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ILibraryAppletCreator<T>(T);

impl ILibraryAppletCreator<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ILibraryAppletCreator<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILibraryAppletCreator(domain)),
			Err((sess, err)) => Err((ILibraryAppletCreator(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILibraryAppletCreator<Session>> {
		Ok(ILibraryAppletCreator(self.0.duplicate()?))
	}
}

impl<T> Deref for ILibraryAppletCreator<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILibraryAppletCreator<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILibraryAppletCreator<T> {
	pub fn create_library_applet(&self, unk0: u32, unk1: u32) -> Result<::nn::am::service::ILibraryAppletAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn terminate_all_library_applets(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn are_any_library_applets_left(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn create_storage(&self, unk0: i64) -> Result<::nn::am::service::IStorage<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn create_transfer_memory_storage(&self, unk0: bool, unk1: i64, unk2: &KObject) -> Result<::nn::am::service::IStorage<T>> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i64,
		}
		let req = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
			})
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn create_handle_storage(&self, unk0: i64, unk1: &KObject) -> Result<::nn::am::service::IStorage<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for ILibraryAppletCreator<T> {
	fn from(obj: T) -> ILibraryAppletCreator<T> {
		ILibraryAppletCreator(obj)
	}
}
