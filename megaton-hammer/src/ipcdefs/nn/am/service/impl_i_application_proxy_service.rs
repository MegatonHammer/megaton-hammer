
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IApplicationProxyService<T>(T);

impl IApplicationProxyService<Session> {
	pub fn raw_new() -> Result<IApplicationProxyService<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"appletOE")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IApplicationProxyService<Session>>> {
		use alloc::arc::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IApplicationProxyService<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"appletOE") {
			let ret = Arc::new(IApplicationProxyService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IApplicationProxyService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IApplicationProxyService(domain)),
			Err((sess, err)) => Err((IApplicationProxyService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IApplicationProxyService<Session>> {
		Ok(IApplicationProxyService(self.0.duplicate()?))
	}
}

impl<T> Deref for IApplicationProxyService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IApplicationProxyService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IApplicationProxyService<T> {
	pub fn open_application_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::ipcdefs::nn::am::service::IApplicationProxy<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(0)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IApplicationProxyService<T> {
	fn from(obj: T) -> IApplicationProxyService<T> {
		IApplicationProxyService(obj)
	}
}
