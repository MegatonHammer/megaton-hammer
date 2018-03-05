
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

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
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown3(&self, unk0: u64) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown4(&self, unk0: u64) -> Result<()> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown7(&self, unk0: u64, unk1: [u8; 0x58]) -> Result<()> {
		let req = Request::new(7)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown10(&self, ) -> Result<u32> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown11(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown12(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(12)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn Unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IUser {
	unsafe fn from_kobject(obj: KObject) -> IUser {
		IUser(Session::from_kobject(obj))
	}
}
