
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAudioController(Session);

impl IAudioController {
	pub fn Unknown0(&self, unk0: u32) -> Result<u32> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown1(&self, unk0: u32, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
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
	pub fn Unknown3(&self, ) -> Result<u32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown4(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown5(&self, unk0: u8, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u32,
		}
		let req = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown6(&self, ) -> Result<()> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown7(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown8(&self, ) -> Result<u32> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown9(&self, unk0: u32) -> Result<u32> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown10(&self, unk0: u32, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(10)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown11(&self, unk0: u32) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown12(&self, ) -> Result<u32> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown13(&self, unk0: u32) -> Result<u32> {
		let req = Request::new(13)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown14(&self, unk0: u32, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(14)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown15(&self, unk0: u32) -> Result<()> {
		let req = Request::new(15)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown16(&self, unk0: u8) -> Result<()> {
		let req = Request::new(16)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown17(&self, unk0: u32) -> Result<()> {
		let req = Request::new(17)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown18(&self, ) -> Result<u32> {
		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn Unknown19(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown20(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown21(&self, ) -> Result<u32> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown22(&self, ) -> Result<()> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IAudioController {
	unsafe fn from_kobject(obj: KObject) -> IAudioController {
		IAudioController(Session::from_kobject(obj))
	}
}
