
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IProgressMonitorForDeleteUserSaveDataAll(Session);

impl AsRef<Session> for IProgressMonitorForDeleteUserSaveDataAll {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IProgressMonitorForDeleteUserSaveDataAll {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown1(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown2(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IProgressMonitorForDeleteUserSaveDataAll {
	unsafe fn from_kobject(obj: KObject) -> IProgressMonitorForDeleteUserSaveDataAll {
		IProgressMonitorForDeleteUserSaveDataAll(Session::from_kobject(obj))
	}
}
