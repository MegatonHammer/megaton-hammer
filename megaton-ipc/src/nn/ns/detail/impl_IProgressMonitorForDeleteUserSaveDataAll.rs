
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IProgressMonitorForDeleteUserSaveDataAll(Session);

impl IProgressMonitorForDeleteUserSaveDataAll {
	pub fn Unknown0(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown1(&self, ) -> Result<u8> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown2(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IProgressMonitorForDeleteUserSaveDataAll {
	unsafe fn from_kobject(obj: KObject) -> IProgressMonitorForDeleteUserSaveDataAll {
		IProgressMonitorForDeleteUserSaveDataAll(Session::from_kobject(obj))
	}
}
