
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAllSystemAppletProxiesService(Session);

impl IAllSystemAppletProxiesService {
	pub fn OpenSystemAppletProxy(&self, unk0: u64, unk2: KObject) -> Result<(::nn::am::service::ISystemAppletProxy)> {
		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn OpenLibraryAppletProxyOld(&self, unk0: u64, unk2: KObject) -> Result<(::nn::am::service::ILibraryAppletProxy)> {
		let req = Request::new(200)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn OpenLibraryAppletProxy(&self, unk0: u64, unk2: KObject, unk3: &::nn::am::AppletAttribute) -> Result<(::nn::am::service::ILibraryAppletProxy)> {
		let req = Request::new(201)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenOverlayAppletProxy(&self, unk0: u64, unk2: KObject) -> Result<(::nn::am::service::IOverlayAppletProxy)> {
		let req = Request::new(300)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenSystemApplicationProxy(&self, unk0: u64, unk2: KObject) -> Result<(::nn::am::service::IApplicationProxy)> {
		let req = Request::new(350)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateSelfLibraryAppletCreatorForDevelop(&self, unk0: u64) -> Result<(::nn::am::service::ILibraryAppletCreator)> {
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
