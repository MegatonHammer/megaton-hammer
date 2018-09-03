
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IContentStorage<T>(T);

impl IContentStorage<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IContentStorage<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IContentStorage(domain)),
			Err((sess, err)) => Err((IContentStorage(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IContentStorage<Session>> {
		Ok(IContentStorage(self.0.duplicate()?))
	}
}

impl<T> Deref for IContentStorage<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IContentStorage<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IContentStorage<T> {
	pub fn generate_place_holder_id(&self, ) -> Result<u128> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn create_place_holder(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_place_holder(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn has_place_holder(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_place_holder(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn register(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn has(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_place_holder_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cleanup_all_place_holder(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_place_holder(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_content_count(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn list_content_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn disable_forcibly(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn revert_to_place_holder(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_place_holder_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_content_id_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_rights_id_from_place_holder_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_rights_id_from_content_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_content_for_debug(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn get_free_space_size(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_total_space_size(&self, ) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn flush_storage(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(24)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown25(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown26(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IContentStorage<T> {
	fn from(obj: T) -> IContentStorage<T> {
		IContentStorage(obj)
	}
}
