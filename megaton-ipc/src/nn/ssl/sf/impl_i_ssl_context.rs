
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISslContext(Session);

impl AsRef<Session> for ISslContext {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISslContext {
	pub fn set_option(&self, unk0: ::nn::ssl::sf::ContextOption, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::ssl::sf::ContextOption,
			unk1: i32,
		}
		let req = Request::new(0)
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

		let req = Request::new(1)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn create_connection(&self, ) -> Result<::nn::ssl::sf::ISslConnection> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn get_connection_count(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn import_server_pki(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn import_client_pki(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn remove_server_pki(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn remove_client_pki(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_internal_pki(&self, unk0: ::nn::ssl::sf::InternalPki) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn add_policy_oid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn import_crl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn remove_crl(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISslContext {
	unsafe fn from_kobject(obj: KObject) -> ISslContext {
		ISslContext(Session::from_kobject(obj))
	}
}
