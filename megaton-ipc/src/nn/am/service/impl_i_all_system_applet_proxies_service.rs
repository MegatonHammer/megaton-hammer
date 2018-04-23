
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAllSystemAppletProxiesService<T>(T);

impl IAllSystemAppletProxiesService<Session> {
	pub fn new() -> Result<Arc<IAllSystemAppletProxiesService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAllSystemAppletProxiesService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"appletAE") {
			let ret = Arc::new(IAllSystemAppletProxiesService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"appletAE").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IAllSystemAppletProxiesService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IAllSystemAppletProxiesService(domain)),
			Err((sess, err)) => Err((IAllSystemAppletProxiesService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IAllSystemAppletProxiesService<Session>> {
		Ok(IAllSystemAppletProxiesService(self.0.duplicate()?))
	}
}

impl<T> Deref for IAllSystemAppletProxiesService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IAllSystemAppletProxiesService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IAllSystemAppletProxiesService<T> {
	pub fn open_system_applet_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::ISystemAppletProxy<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn open_library_applet_proxy_old(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::ILibraryAppletProxy<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn open_library_applet_proxy(&self, unk0: u64, unk2: &KObject, unk3: &::nn::am::AppletAttribute) -> Result<::nn::am::service::ILibraryAppletProxy<T>> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(201)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			.descriptor(IPCBuffer::from_ref(unk3, 0x15))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_overlay_applet_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::IOverlayAppletProxy<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(300)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_system_application_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::IApplicationProxy<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(350)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn create_self_library_applet_creator_for_develop(&self, unk0: u64) -> Result<::nn::am::service::ILibraryAppletCreator<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(400)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IAllSystemAppletProxiesService<T> {
	fn from(obj: T) -> IAllSystemAppletProxiesService<T> {
		IAllSystemAppletProxiesService(obj)
	}
}
