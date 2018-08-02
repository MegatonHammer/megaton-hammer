
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IFileSystemProxyForLoader<T>(T);

impl IFileSystemProxyForLoader<Session> {
	pub fn raw_new() -> Result<IFileSystemProxyForLoader<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"fsp-ldr\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IFileSystemProxyForLoader<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IFileSystemProxyForLoader<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"fsp-ldr\0") {
			let ret = Arc::new(IFileSystemProxyForLoader(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IFileSystemProxyForLoader<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFileSystemProxyForLoader(domain)),
			Err((sess, err)) => Err((IFileSystemProxyForLoader(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFileSystemProxyForLoader<Session>> {
		Ok(IFileSystemProxyForLoader(self.0.duplicate()?))
	}
}

impl<T> Deref for IFileSystemProxyForLoader<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFileSystemProxyForLoader<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFileSystemProxyForLoader<T> {
	pub fn mount_code(&self, tid: ::ipcdefs::nn::ApplicationId, content_path: &i8) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(tid)
			.descriptor(IPCBuffer::from_ref(content_path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn is_code_mounted(&self, tid: ::ipcdefs::nn::ApplicationId) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(tid)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IFileSystemProxyForLoader<T> {
	fn from(obj: T) -> IFileSystemProxyForLoader<T> {
		IFileSystemProxyForLoader(obj)
	}
}
