
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHOSBinderDriver<T>(T);

impl IHOSBinderDriver<Session> {
	pub fn raw_new() -> Result<IHOSBinderDriver<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let r = sm.get_service(*b"dispdrv\0").map(|s: KObject| Session::from(s).into());
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}

	pub fn new() -> Result<Arc<IHOSBinderDriver<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IHOSBinderDriver<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"dispdrv\0") {
			let ret = Arc::new(IHOSBinderDriver(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IHOSBinderDriver<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHOSBinderDriver(domain)),
			Err((sess, err)) => Err((IHOSBinderDriver(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHOSBinderDriver<Session>> {
		Ok(IHOSBinderDriver(self.0.duplicate()?))
	}
}

impl<T> Deref for IHOSBinderDriver<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHOSBinderDriver<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHOSBinderDriver<T> {
	pub fn transact_parcel(&self, id: i32, code: u32, flags: u32, parcel_data: &[u8], parcel_reply: &mut [u8]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			id: i32,
			code: u32,
			flags: u32,
		}
		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				id,
				code,
				flags,
			})
			.descriptor(IPCBuffer::from_slice(parcel_data, 5))
			.descriptor(IPCBuffer::from_mut_slice(parcel_reply, 6))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn adjust_refcount(&self, unk0: i32, unk1: i32, unk2: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_native_handle(&self, unk0: i32, unk1: u32) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn transact_parcel_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for IHOSBinderDriver<T> {
	fn from(obj: T) -> IHOSBinderDriver<T> {
		IHOSBinderDriver(obj)
	}
}
