
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct ILdrShellInterface<T>(T);

impl ILdrShellInterface<Session> {
	pub fn raw_new() -> Result<ILdrShellInterface<Session>> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"ldr:shel")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<ILdrShellInterface<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<ILdrShellInterface<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"ldr:shel") {
			let ret = Arc::new(ILdrShellInterface(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<ILdrShellInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ILdrShellInterface(domain)),
			Err((sess, err)) => Err((ILdrShellInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ILdrShellInterface<Session>> {
		Ok(ILdrShellInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for ILdrShellInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ILdrShellInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ILdrShellInterface<T> {
	pub fn add_process_to_launch_queue(&self, unk0: &[u8; 0x200], size: u32, app_id: ::nn::ncm::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			app_id: ::nn::ncm::ApplicationId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				size,
				app_id,
			})
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_launch_queue(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ILdrShellInterface<T> {
	fn from(obj: T) -> ILdrShellInterface<T> {
		ILdrShellInterface(obj)
	}
}
