
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISocket(Session);

impl ISocket {
	pub fn Unknown0(&self, ) -> Result<(u32, u32)> {
		let req = Request::new(0)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}
	pub fn Unknown1(&self, unk0: [u8; 0x42]) -> Result<(u32, u32)> {
		let req = Request::new(1)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}
	pub fn Unknown2(&self, unk0: [u8; 0x42]) -> Result<(u32, u32)> {
		let req = Request::new(2)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}
	pub fn Unknown3(&self, unk0: u32) -> Result<(u32, u32)> {
		let req = Request::new(3)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}
	pub fn Unknown4(&self, ) -> Result<([u8; 0x42], u32, Session)> {
		let req = Request::new(4)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: [u8; 0x42],
			unk1: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}
	// fn Unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown7(&self, unk0: u32) -> Result<(u32, u32)> {
		let req = Request::new(7)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}
	pub fn Unknown8(&self, unk0: u32, unk1: u32) -> Result<(u32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: u32,
			unk3: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}
	// fn Unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown10(&self, unk0: u32) -> Result<([u8; 0x42], u32, Session)> {
		let req = Request::new(10)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: [u8; 0x42],
			unk2: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}
	// fn Unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown14(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown16(&self, unk0: u32) -> Result<(u32, u64)> {
		let req = Request::new(16)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}
}

impl FromKObject for ISocket {
	unsafe fn from_kobject(obj: KObject) -> ISocket {
		ISocket(Session::from_kobject(obj))
	}
}
