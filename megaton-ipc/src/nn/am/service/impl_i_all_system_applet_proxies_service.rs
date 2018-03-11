
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAllSystemAppletProxiesService(Session);

impl IAllSystemAppletProxiesService {
	pub fn new() -> Result<IAllSystemAppletProxiesService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"appletAE").map(|s| unsafe { IAllSystemAppletProxiesService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAllSystemAppletProxiesService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAllSystemAppletProxiesService {
	pub fn open_system_applet_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::ISystemAppletProxy> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn open_library_applet_proxy_old(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::ILibraryAppletProxy> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn open_library_applet_proxy(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn open_overlay_applet_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::IOverlayAppletProxy> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(300)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_system_application_proxy(&self, unk0: u64, unk2: &KObject) -> Result<::nn::am::service::IApplicationProxy> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(350)
			.args(unk0)
			.send_pid()
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_self_library_applet_creator_for_develop(&self, unk0: u64) -> Result<::nn::am::service::ILibraryAppletCreator> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(400)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IAllSystemAppletProxiesService {
	unsafe fn from_kobject(obj: KObject) -> IAllSystemAppletProxiesService {
		IAllSystemAppletProxiesService(Session::from_kobject(obj))
	}
}
