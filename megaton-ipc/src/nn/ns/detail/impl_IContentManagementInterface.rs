
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IContentManagementInterface(Session);

impl IContentManagementInterface {
	pub fn Unknown11(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown43(&self, ) -> Result<()> {
		let req = Request::new(43)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown47(&self, ) -> Result<()> {
		let req = Request::new(47)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown48(&self, ) -> Result<()> {
		let req = Request::new(48)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown600(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(600)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown601(&self, ) -> Result<()> {
		let req = Request::new(601)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown605(&self, ) -> Result<()> {
		let req = Request::new(605)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown607(&self, ) -> Result<u8> {
		let req = Request::new(607)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IContentManagementInterface {
	unsafe fn from_kobject(obj: KObject) -> IContentManagementInterface {
		IContentManagementInterface(Session::from_kobject(obj))
	}
}
