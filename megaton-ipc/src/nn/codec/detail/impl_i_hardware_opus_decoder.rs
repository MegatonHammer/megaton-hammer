
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IHardwareOpusDecoder(Session);

impl AsRef<Session> for IHardwareOpusDecoder {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHardwareOpusDecoder {
	// fn decode_interleaved(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_context(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn decode_interleaved_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_context_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown4(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown5(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IHardwareOpusDecoder {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoder {
		IHardwareOpusDecoder(Session::from_kobject(obj))
	}
}
