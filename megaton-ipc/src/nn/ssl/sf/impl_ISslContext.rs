
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct ISslContext(Session);

impl AsRef<Session> for ISslContext {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISslContext {
	pub fn SetOption(&self, unk0: ::nn::ssl::sf::ContextOption, unk1: i32) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetOption(&self, unk0: ::nn::ssl::sf::ContextOption) -> Result<i32> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CreateConnection(&self, ) -> Result<::nn::ssl::sf::ISslConnection> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetConnectionCount(&self, ) -> Result<u32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn ImportServerPki(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ImportClientPki(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RemoveServerPki(&self, unk0: u64) -> Result<()> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RemoveClientPki(&self, unk0: u64) -> Result<()> {
		let req = Request::new(7)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RegisterInternalPki(&self, unk0: ::nn::ssl::sf::InternalPki) -> Result<u64> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn AddPolicyOid(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ImportCrl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RemoveCrl(&self, unk0: u64) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISslContext {
	unsafe fn from_kobject(obj: KObject) -> ISslContext {
		ISslContext(Session::from_kobject(obj))
	}
}
