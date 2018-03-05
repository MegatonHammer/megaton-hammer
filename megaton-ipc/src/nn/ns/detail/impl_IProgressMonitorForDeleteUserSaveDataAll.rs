
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IProgressMonitorForDeleteUserSaveDataAll(Session);

impl IProgressMonitorForDeleteUserSaveDataAll {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
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
	pub fn Unknown10(&self, ) -> Result<[u8; 0x28]> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<[u8; 0x28]> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IProgressMonitorForDeleteUserSaveDataAll {
	unsafe fn from_kobject(obj: KObject) -> IProgressMonitorForDeleteUserSaveDataAll {
		IProgressMonitorForDeleteUserSaveDataAll(Session::from_kobject(obj))
	}
}
