
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ISocket(Session);

impl AsRef<Session> for ISocket {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
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

	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn Unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	pub fn Unknown9(&self, ) -> Result<(u32, KObject)> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	// fn Unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown11(&self, unk0: u32, unk1: u32) -> Result<(u32, KObject)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	// fn Unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown13(&self, unk0: u32, unk1: u32, unk2: u32, unk3: u64, unk4: &KObject) -> Result<(u32, KObject)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
			unk3: u64,
		}
		let req = Request::new(13)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.copy_handle(unk4)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

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
