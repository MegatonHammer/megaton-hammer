
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IQueryService(Session);

impl IQueryService {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown4(&self, unk0: u64) -> Result<(u64, u64, u64, u64, u64)> {
		let req = Request::new(4)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u64,
			unk2: u64,
			unk3: u64,
			unk4: u64,
			unk5: u64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone(),res.get_raw().unk4.clone(),res.get_raw().unk5.clone()))
	}
	pub fn Unknown5(&self, unk0: u64, unk1: u64, unk2: u64) -> Result<(u64, u64, u64, u64, u64)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
			unk2: u64,
		}
		let req = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk3: u64,
			unk4: u64,
			unk5: u64,
			unk6: u64,
			unk7: u64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk3.clone(),res.get_raw().unk4.clone(),res.get_raw().unk5.clone(),res.get_raw().unk6.clone(),res.get_raw().unk7.clone()))
	}
	pub fn Unknown6(&self, unk0: u64, unk1: u64) -> Result<(u64, u64, u64, u64, u64)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: u64,
			unk3: u64,
			unk4: u64,
			unk5: u64,
			unk6: u64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone(),res.get_raw().unk4.clone(),res.get_raw().unk5.clone(),res.get_raw().unk6.clone()))
	}
	// fn Unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown9(&self, ) -> Result<(u32, u32, u32)> {
		let req = Request::new(9)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}
	// fn Unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IQueryService {
	unsafe fn from_kobject(obj: KObject) -> IQueryService {
		IQueryService(Session::from_kobject(obj))
	}
}
