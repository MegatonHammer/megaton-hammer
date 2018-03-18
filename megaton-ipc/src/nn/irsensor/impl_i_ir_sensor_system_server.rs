
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IIrSensorSystemServer(Session);

impl IIrSensorSystemServer {
	pub fn new() -> Result<Arc<IIrSensorSystemServer>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IIrSensorSystemServer>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"irs:sys\0").map(|s| Arc::new(unsafe { IIrSensorSystemServer::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IIrSensorSystemServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IIrSensorSystemServer {
	pub fn set_applet_resource_user_id(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(500)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_applet_resource_user_id(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(501)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unregister_applet_resource_user_id(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(502)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_applet_to_get_input(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(503)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IIrSensorSystemServer {
	unsafe fn from_kobject(obj: KObject) -> IIrSensorSystemServer {
		IIrSensorSystemServer(Session::from_kobject(obj))
	}
}
