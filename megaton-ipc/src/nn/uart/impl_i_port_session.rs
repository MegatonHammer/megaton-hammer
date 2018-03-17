
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
	pub fn open_session(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_session_for_test(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown3(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown5(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown6(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

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
