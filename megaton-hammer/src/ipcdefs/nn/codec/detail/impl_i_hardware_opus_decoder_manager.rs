
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IHardwareOpusDecoderManager<T>(T);

impl IHardwareOpusDecoderManager<Session> {
	pub fn raw_new() -> Result<IHardwareOpusDecoderManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"hwopus\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IHardwareOpusDecoderManager<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IHardwareOpusDecoderManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"hwopus\0\0") {
			let ret = Arc::new(IHardwareOpusDecoderManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IHardwareOpusDecoderManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IHardwareOpusDecoderManager(domain)),
			Err((sess, err)) => Err((IHardwareOpusDecoderManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IHardwareOpusDecoderManager<Session>> {
		Ok(IHardwareOpusDecoderManager(self.0.duplicate()?))
	}
}

impl<T> Deref for IHardwareOpusDecoderManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IHardwareOpusDecoderManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IHardwareOpusDecoderManager<T> {
	pub fn get_hardware_opus_decoder(&self, unk0: u64, unk1: u32, unk2: &KObject) -> Result<T> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_work_buffer_size(&self, unk0: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn initialize_ex(&self, unk0: u32, unk1: &KObject, unk2: &[u8; 0x110]) -> Result<T> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 1], [_; 0]> = Request::new(2)
			.args(unk0)
			.copy_handle(unk1)
			.descriptor(IPCBuffer::from_ref(unk2, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_work_buffer_size_ex(&self, unk0: &[u8; 0x110]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IHardwareOpusDecoderManager<T> {
	fn from(obj: T) -> IHardwareOpusDecoderManager<T> {
		IHardwareOpusDecoderManager(obj)
	}
}
