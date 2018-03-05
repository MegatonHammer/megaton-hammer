
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISslService(Session);

impl ISslService {
	pub fn CreateContext(&self, unk0: ::nn::ssl::sf::SslVersion, unk1: u64) -> Result<::nn::ssl::sf::ISslContext> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::ssl::sf::SslVersion,
			unk1: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn GetContextCount(&self, ) -> Result<u32> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetCertificates(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetCertificateBufSize(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn DebugIoctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetInterfaceVersion(&self, unk0: u32) -> Result<()> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for ISslService {
	unsafe fn from_kobject(obj: KObject) -> ISslService {
		ISslService(Session::from_kobject(obj))
	}
}
