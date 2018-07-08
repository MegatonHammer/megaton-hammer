
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAddOnContentManager<T>(T);

impl IAddOnContentManager<Session> {
	pub fn raw_new() -> Result<IAddOnContentManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"aoc:u\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IAddOnContentManager<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IAddOnContentManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"aoc:u\0\0\0") {
			let ret = Arc::new(IAddOnContentManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IAddOnContentManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAddOnContentManager(domain)),
			Err((sess, err)) => Err((IAddOnContentManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAddOnContentManager<Session>> {
		Ok(IAddOnContentManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IAddOnContentManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAddOnContentManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAddOnContentManager<T> {
	pub fn count_add_on_content_by_application_id(&self, unk0: ::ipcdefs::nn::ncm::ApplicationId) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn list_add_on_content_by_application_id(&self, unk0: i32, unk1: i32, unk2: ::ipcdefs::nn::ncm::ApplicationId, unk4: &mut [i32]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: ::ipcdefs::nn::ncm::ApplicationId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk4, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn count_add_on_content(&self, unk0: u64) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			.send_pid()
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn list_add_on_content(&self, unk0: i32, unk1: i32, unk2: u64, unk5: &mut [i32]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_mut_slice(unk5, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_add_on_content_base_id_by_application_id(&self, unk0: ::ipcdefs::nn::ncm::ApplicationId) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_add_on_content_base_id(&self, unk0: u64) -> Result<u64> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(unk0)
			.send_pid()
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn prepare_add_on_content_by_application_id(&self, unk0: i32, unk1: ::ipcdefs::nn::ncm::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::ncm::ApplicationId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn prepare_add_on_content(&self, unk0: i32, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IAddOnContentManager<T> {
	fn from(obj: T) -> IAddOnContentManager<T> {
		IAddOnContentManager(obj)
	}
}
