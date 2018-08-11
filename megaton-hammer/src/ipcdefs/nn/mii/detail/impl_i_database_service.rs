
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDatabaseService<T>(T);

impl IDatabaseService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDatabaseService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDatabaseService(domain)),
			Err((sess, err)) => Err((IDatabaseService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDatabaseService<Session>> {
		Ok(IDatabaseService(self.0.duplicate()?))
	}
}

impl<T> Deref for IDatabaseService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDatabaseService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDatabaseService<T> {
	pub fn is_updated(&self, unk0: i32) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_full_database(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_count(&self, unk0: i32) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get(&self, unk0: i32, unk2: &mut [::ipcdefs::nn::mii::CharInfoElement]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get1(&self, unk0: i32, unk2: &mut [::ipcdefs::nn::mii::CharInfo]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(4)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn update_latest(&self, unk0: ::ipcdefs::nn::mii::CharInfo, unk1: i32) -> Result<::ipcdefs::nn::mii::CharInfo> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::mii::CharInfo,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::ipcdefs::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn build_random(&self, unk0: i32, unk1: i32, unk2: i32) -> Result<::ipcdefs::nn::mii::CharInfo> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let res : Response<::ipcdefs::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn build_default(&self, unk0: i32) -> Result<::ipcdefs::nn::mii::CharInfo> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::mii::CharInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get2(&self, unk0: i32, unk2: &mut [::ipcdefs::nn::mii::StoreDataElement]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(8)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get3(&self, unk0: i32, unk2: &mut [::ipcdefs::nn::mii::StoreData]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(9)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn update_latest1(&self, unk0: ::ipcdefs::nn::mii::StoreData, unk1: i32) -> Result<::ipcdefs::nn::mii::StoreData> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::mii::StoreData,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::ipcdefs::nn::mii::StoreData> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn find_index(&self, unk0: ::ipcdefs::nn::mii::CreateId, unk1: bool) -> Result<i32> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::mii::CreateId,
			unk1: bool,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn _move(&self, unk0: ::ipcdefs::nn::mii::CreateId, unk1: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::mii::CreateId,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn add_or_replace(&self, unk0: ::ipcdefs::nn::mii::StoreData) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete(&self, unk0: ::ipcdefs::nn::mii::CreateId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(14)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn destroy_file(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_file(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn format(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(17)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn import(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn export(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_broken_database_with_clear_flag(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_index(&self, unk0: ::ipcdefs::nn::mii::CharInfo) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(21)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn set_interface_version(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn convert(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDatabaseService<T> {
	fn from(obj: T) -> IDatabaseService<T> {
		IDatabaseService(obj)
	}
}
