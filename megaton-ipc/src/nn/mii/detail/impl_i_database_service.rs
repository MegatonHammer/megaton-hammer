
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDatabaseService(Session);

impl AsRef<Session> for IDatabaseService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDatabaseService {
	pub fn is_updated(&self, unk0: i32) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_full_database(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_count(&self, unk0: i32) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn update_latest(&self, unk0: ::nn::mii::CharInfo, unk1: i32) -> Result<::nn::mii::CharInfo> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn build_random(&self, unk0: i32, unk1: i32, unk2: i32) -> Result<::nn::mii::CharInfo> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn build_default(&self, unk0: i32) -> Result<::nn::mii::CharInfo> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(unk0)
			;
		let res : Response<::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn update_latest1(&self, unk0: ::nn::mii::StoreData, unk1: i32) -> Result<::nn::mii::StoreData> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<::nn::mii::StoreData> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn find_index(&self, unk0: ::nn::mii::CreateId, unk1: bool) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn _move(&self, unk0: ::nn::mii::CreateId, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn add_or_replace(&self, unk0: ::nn::mii::StoreData) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete(&self, unk0: ::nn::mii::CreateId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn destroy_file(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_file(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn format(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn import(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn export(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_broken_database_with_clear_flag(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_index(&self, unk0: ::nn::mii::CharInfo) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDatabaseService {
	unsafe fn from_kobject(obj: KObject) -> IDatabaseService {
		IDatabaseService(Session::from_kobject(obj))
	}
}
