
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IUser(Session);

impl IUser {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown1(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2(&self, ) -> Result<u32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown3(&self, ) -> Result<u8> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IUser {
	unsafe fn from_kobject(obj: KObject) -> IUser {
		IUser(Session::from_kobject(obj))
	}
}
