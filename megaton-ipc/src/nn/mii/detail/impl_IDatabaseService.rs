
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDatabaseService(Session);

impl IDatabaseService {
	pub fn IsUpdated(&self, unk0: i32) -> Result<bool> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn IsFullDatabase(&self, ) -> Result<bool> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetCount(&self, unk0: i32) -> Result<i32> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Get(&self, unk0: i32, unk2: &mut [::nn::mii::CharInfoElement]) -> Result<i32> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Get1(&self, unk0: i32, unk2: &mut [::nn::mii::CharInfo]) -> Result<i32> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn UpdateLatest(&self, unk0: ::nn::mii::CharInfo, unk1: i32) -> Result<::nn::mii::CharInfo> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::mii::CharInfo,
			unk1: i32,
		}
		let req = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn BuildRandom(&self, unk0: i32, unk1: i32, unk2: i32) -> Result<::nn::mii::CharInfo> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: i32,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn BuildDefault(&self, unk0: i32) -> Result<::nn::mii::CharInfo> {
		let req = Request::new(7)
			.args(unk0)
			;
		let mut res : Response<::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Get2(&self, unk0: i32, unk2: &mut [::nn::mii::StoreDataElement]) -> Result<i32> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Get3(&self, unk0: i32, unk2: &mut [::nn::mii::StoreData]) -> Result<i32> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn UpdateLatest1(&self, unk0: ::nn::mii::StoreData, unk1: i32) -> Result<::nn::mii::StoreData> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::mii::StoreData,
			unk1: i32,
		}
		let req = Request::new(10)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<::nn::mii::StoreData> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn FindIndex(&self, unk0: ::nn::mii::CreateId, unk1: bool) -> Result<i32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::mii::CreateId,
			unk1: bool,
		}
		let req = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Move(&self, unk0: ::nn::mii::CreateId, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::mii::CreateId,
			unk1: i32,
		}
		let req = Request::new(12)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn AddOrReplace(&self, unk0: ::nn::mii::StoreData) -> Result<()> {
		let req = Request::new(13)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Delete(&self, unk0: ::nn::mii::CreateId) -> Result<()> {
		let req = Request::new(14)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DestroyFile(&self, ) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DeleteFile(&self, ) -> Result<()> {
		let req = Request::new(16)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Format(&self, ) -> Result<()> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Import(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Export(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn IsBrokenDatabaseWithClearFlag(&self, ) -> Result<bool> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetIndex(&self, unk0: ::nn::mii::CharInfo) -> Result<i32> {
		let req = Request::new(21)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IDatabaseService {
	unsafe fn from_kobject(obj: KObject) -> IDatabaseService {
		IDatabaseService(Session::from_kobject(obj))
	}
}
