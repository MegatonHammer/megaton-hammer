
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILocalManager<T>(T);

impl ILocalManager<Session> {
	pub fn raw_new() -> Result<ILocalManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"wlan:lcl")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ILocalManager<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<ILocalManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"wlan:lcl") {
			let ret = Arc::new(ILocalManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ILocalManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILocalManager(domain)),
			Err((sess, err)) => Err((ILocalManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILocalManager<Session>> {
		Ok(ILocalManager(self.0.duplicate()?))
	}
}

impl<T> Deref for ILocalManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILocalManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILocalManager<T> {
	pub fn unknown0(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown1(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown3(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown5(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown8(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown10(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown12(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown14(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(14)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown16(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown17(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown18(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown19(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown20(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown21(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown22(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown23(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown24(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown25(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown26(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown27(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(27)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown28(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown29(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(29)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown30(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown31(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown32(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown33(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown34(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown35(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown36(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown37(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(37)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown38(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown39(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(39)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown40(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown41(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown42(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(42)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown43(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(43)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown44(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(44)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ILocalManager<T> {
	fn from(obj: T) -> ILocalManager<T> {
		ILocalManager(obj)
	}
}
