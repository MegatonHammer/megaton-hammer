
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISslContext<T>(T);

impl ISslContext<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISslContext<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISslContext(domain)),
			Err((sess, err)) => Err((ISslContext(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISslContext<Session>> {
		Ok(ISslContext(self.0.duplicate()?))
	}
}

impl<T> Deref for ISslContext<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISslContext<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISslContext<T> {
	pub fn set_option(&self, unk0: ::nn::ssl::sf::ContextOption, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::ssl::sf::ContextOption,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_option(&self, unk0: ::nn::ssl::sf::ContextOption) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn create_connection(&self, ) -> Result<::nn::ssl::sf::ISslConnection<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_connection_count(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn import_server_pki(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn import_client_pki(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn remove_server_pki(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn remove_client_pki(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_internal_pki(&self, unk0: ::nn::ssl::sf::InternalPki) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn add_policy_oid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn import_crl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn remove_crl(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISslContext<T> {
	fn from(obj: T) -> ISslContext<T> {
		ISslContext(obj)
	}
}
