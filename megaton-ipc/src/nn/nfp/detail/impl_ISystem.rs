
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct ISystem(Session);

impl AsRef<Session> for ISystem {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystem {
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

	pub fn Unknown23(&self, ) -> Result<KObject> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown100(&self, unk0: u64) -> Result<()> {
		let req = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown101(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown102(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown103(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown104(&self, unk0: u64) -> Result<()> {
		let req = Request::new(104)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown105(&self, unk0: u64) -> Result<()> {
		let req = Request::new(105)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown106(&self, unk0: u64) -> Result<u8> {
		let req = Request::new(106)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for ISystem {
	unsafe fn from_kobject(obj: KObject) -> ISystem {
		ISystem(Session::from_kobject(obj))
	}
}
