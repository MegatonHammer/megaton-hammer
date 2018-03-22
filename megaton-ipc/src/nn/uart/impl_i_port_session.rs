
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IPortSession(Session);

impl AsRef<Session> for IPortSession {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPortSession {
	// fn open_session(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn open_session_for_test(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown2(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown4(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown7(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IPortSession {
	unsafe fn from_kobject(obj: KObject) -> IPortSession {
		IPortSession(Session::from_kobject(obj))
	}
}
