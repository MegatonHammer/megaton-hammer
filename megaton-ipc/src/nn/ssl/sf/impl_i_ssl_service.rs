
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISslService(Session);

impl ISslService {
	pub fn new() -> Result<ISslService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"ssl\0\0\0\0\0").map(|s| unsafe { ISslService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISslService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISslService {
	pub fn create_context(&self, unk0: ::nn::ssl::sf::SslVersion, unk1: u64) -> Result<::nn::ssl::sf::ISslContext> {
		use megaton_hammer::ipc::{Request, Response};

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

	pub fn get_context_count(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_certificates(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_certificate_buf_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn debug_ioctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_interface_version(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISslService {
	unsafe fn from_kobject(obj: KObject) -> ISslService {
		ISslService(Session::from_kobject(obj))
	}
}
