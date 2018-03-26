
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IDsService<T>(T);

impl IDsService<Session> {
	pub fn new() -> Result<Arc<IDsService<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IDsService<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"usb:ds\0\0") {
			let ret = Arc::new(IDsService(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"usb:ds\0\0").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IDsService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDsService(domain)),
			Err((sess, err)) => Err((IDsService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDsService<Session>> {
		Ok(IDsService(self.0.duplicate()?))
	}
}

impl<T> Deref for IDsService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDsService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDsService<T> {
	pub fn bind_device(&self, complex_id: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(complex_id)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn bind_client_process(&self, unk0: &KObject) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.copy_handle(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_ds_interface(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_state_change_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_state(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_vid_pid_bcd(&self, descriptor: &::nn::usb::UsbDescriptorData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			.descriptor(IPCBuffer::from_ref(descriptor, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDsService<T> {
	fn from(obj: T) -> IDsService<T> {
		IDsService(obj)
	}
}
