
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct ISslConnection(Session);

impl AsRef<Session> for ISslConnection {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISslConnection {
	pub fn SetSocketDescriptor(&self, unk0: i32) -> Result<i32> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn SetHostName(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetVerifyOption(&self, unk0: ::nn::ssl::sf::VerifyOption) -> Result<()> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetIoMode(&self, unk0: ::nn::ssl::sf::IoMode) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSocketDescriptor(&self, ) -> Result<i32> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetHostName(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetVerifyOption(&self, ) -> Result<::nn::ssl::sf::VerifyOption> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<::nn::ssl::sf::VerifyOption> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetIoMode(&self, ) -> Result<::nn::ssl::sf::IoMode> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<::nn::ssl::sf::IoMode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DoHandshake(&self, ) -> Result<()> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn DoHandshakeGetServerCert(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Write(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Pending(&self, ) -> Result<i32> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn Peek(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Poll(&self, unk0: ::nn::ssl::sf::PollEvent, unk1: u32) -> Result<::nn::ssl::sf::PollEvent> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::ssl::sf::PollEvent,
			unk1: u32,
		}
		let req = Request::new(14)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<::nn::ssl::sf::PollEvent> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetVerifyCertError(&self, ) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetNeededServerCertBufferSize(&self, ) -> Result<u32> {
		let req = Request::new(16)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetSessionCacheMode(&self, unk0: ::nn::ssl::sf::SessionCacheMode) -> Result<()> {
		let req = Request::new(17)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSessionCacheMode(&self, ) -> Result<::nn::ssl::sf::SessionCacheMode> {
		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<::nn::ssl::sf::SessionCacheMode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn FlushSessionCache(&self, ) -> Result<()> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetRenegotiationMode(&self, unk0: ::nn::ssl::sf::RenegotiationMode) -> Result<()> {
		let req = Request::new(20)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetRenegotiationMode(&self, ) -> Result<::nn::ssl::sf::RenegotiationMode> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<::nn::ssl::sf::RenegotiationMode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetOption(&self, unk0: bool, unk1: ::nn::ssl::sf::OptionType) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::ssl::sf::OptionType,
		}
		let req = Request::new(22)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetOption(&self, unk0: ::nn::ssl::sf::OptionType) -> Result<bool> {
		let req = Request::new(23)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetVerifyCertErrors(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ISslConnection {
	unsafe fn from_kobject(obj: KObject) -> ISslConnection {
		ISslConnection(Session::from_kobject(obj))
	}
}
