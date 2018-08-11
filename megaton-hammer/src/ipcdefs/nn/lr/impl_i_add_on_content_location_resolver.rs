
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IAddOnContentLocationResolver<T>(T);

impl IAddOnContentLocationResolver<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IAddOnContentLocationResolver<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAddOnContentLocationResolver(domain)),
			Err((sess, err)) => Err((IAddOnContentLocationResolver(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAddOnContentLocationResolver<Session>> {
		Ok(IAddOnContentLocationResolver(self.0.duplicate()?))
	}
}

impl<T> Deref for IAddOnContentLocationResolver<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAddOnContentLocationResolver<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAddOnContentLocationResolver<T> {
	// fn resolve_add_on_content_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn register_add_on_content_storage(&self, unk0: u8, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unregister_all_add_on_content_path(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAddOnContentLocationResolver<T> {
	fn from(obj: T) -> IAddOnContentLocationResolver<T> {
		IAddOnContentLocationResolver(obj)
	}
}
