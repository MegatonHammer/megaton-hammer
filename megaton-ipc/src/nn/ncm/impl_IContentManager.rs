
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IContentManager(Session);

impl IContentManager {
	pub fn Unknown0(&self, unk0: u8) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown1(&self, unk0: u8) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown2(&self, unk0: u8) -> Result<()> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown3(&self, unk0: u8) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetIContentStorage(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetIContentMetaDatabase(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown8(&self, unk0: u8) -> Result<()> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn InitializeStorageForMediaId(&self, unk0: u8) -> Result<()> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn UninitializeStorageForMediaId(&self, unk0: u8) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn InitializeDatabaseForMediaId(&self, unk0: u8) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn UninitializeDatabaseForMediaId(&self, unk0: u8) -> Result<()> {
		let req = Request::new(12)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IContentManager {
	unsafe fn from_kobject(obj: KObject) -> IContentManager {
		IContentManager(Session::from_kobject(obj))
	}
}
