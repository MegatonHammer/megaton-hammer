
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IRegisteredLocationResolver<T>(T);

impl IRegisteredLocationResolver<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IRegisteredLocationResolver<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IRegisteredLocationResolver(domain)),
			Err((sess, err)) => Err((IRegisteredLocationResolver(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IRegisteredLocationResolver<Session>> {
		Ok(IRegisteredLocationResolver(self.0.duplicate()?))
	}
}

impl<T> Deref for IRegisteredLocationResolver<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IRegisteredLocationResolver<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IRegisteredLocationResolver<T> {
	// fn get_patch_type0_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn register_patch_type0_fallback_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unregister_patch_type0_fallback_path(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_patch_type0_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_patch_type1_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn register_patch_type1_fallback_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn unregister_patch_type1_fallback_path(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_patch_type1_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IRegisteredLocationResolver<T> {
	fn from(obj: T) -> IRegisteredLocationResolver<T> {
		IRegisteredLocationResolver(obj)
	}
}
