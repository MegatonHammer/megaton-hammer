
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDebug(Session);

impl IDebug {
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
	pub fn Unknown13(&self, unk0: u64, unk1: [u8; 0x58]) -> Result<()> {
		let req = Request::new(13)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown14(&self, unk0: u64, unk1: [u8; 0x100]) -> Result<()> {
		let req = Request::new(14)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown15(&self, unk0: u64, unk1: [u8; 0x40]) -> Result<()> {
		let req = Request::new(15)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown16(&self, unk0: u64, unk1: [u8; 0x40]) -> Result<()> {
		let req = Request::new(16)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown17(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown18(&self, UNKNOWN) -> Result<UNKNOWN>;
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
	// fn Unknown23(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown24(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown100(&self, unk0: u64) -> Result<()> {
		let req = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown101(&self, unk0: u64, unk1: [u8; 0x40]) -> Result<()> {
		let req = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown102(&self, unk0: u64, unk1: [u8; 0x100]) -> Result<()> {
		let req = Request::new(102)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown103(&self, unk0: u64, unk1: [u8; 0x100]) -> Result<()> {
		let req = Request::new(103)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
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
	pub fn Unknown200(&self, unk0: u64, unk1: [u8; 0x298]) -> Result<()> {
		let req = Request::new(200)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown201(&self, unk0: u64, unk1: [u8; 0x298]) -> Result<()> {
		let req = Request::new(201)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown202(&self, unk0: u64) -> Result<()> {
		let req = Request::new(202)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown203(&self, unk0: u64, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
		}
		let req = Request::new(203)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown204(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown205(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown206(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown300(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown301(&self, ) -> Result<()> {
		let req = Request::new(301)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown302(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown303(&self, unk0: u64, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
		}
		let req = Request::new(303)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown304(&self, unk0: u64) -> Result<()> {
		let req = Request::new(304)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown305(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown306(&self, unk0: u64, unk1: [u8; 0x58]) -> Result<()> {
		let req = Request::new(306)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown307(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown308(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown309(&self, ) -> Result<u32> {
		let req = Request::new(309)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown310(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(310)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown311(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(311)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown312(&self, unk0: u64) -> Result<()> {
		let req = Request::new(312)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Unknown313(&self, unk0: u64) -> Result<()> {
		let req = Request::new(313)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown314(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IDebug {
	unsafe fn from_kobject(obj: KObject) -> IDebug {
		IDebug(Session::from_kobject(obj))
	}
}
