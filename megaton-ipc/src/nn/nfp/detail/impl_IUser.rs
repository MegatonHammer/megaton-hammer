
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IUser(Session);

impl AsRef<Session> for IUser {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
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

	pub fn Unknown5(&self, unk0: u64, unk1: u32, unk2: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
			unk2: u32,
		}
		let req = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown6(&self, unk0: u64) -> Result<()> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown7(&self, unk0: u64, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
		}
		let req = Request::new(7)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown10(&self, unk0: u64) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown11(&self, unk0: u64) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown14(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown16(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown17(&self, unk0: u64) -> Result<KObject> {
		let req = Request::new(17)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown18(&self, unk0: u64) -> Result<KObject> {
		let req = Request::new(18)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown19(&self, ) -> Result<u32> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown20(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(20)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown21(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(21)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown22(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(22)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown23(&self, ) -> Result<KObject> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn Unknown24(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IUser {
	unsafe fn from_kobject(obj: KObject) -> IUser {
		IUser(Session::from_kobject(obj))
	}
}
