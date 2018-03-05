
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAllSystemAppletProxiesService(Session);

impl IAllSystemAppletProxiesService {
	// fn OpenSystemAppletProxy(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenLibraryAppletProxyOld(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenLibraryAppletProxy(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenOverlayAppletProxy(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenSystemApplicationProxy(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn CreateSelfLibraryAppletCreatorForDevelop(&self, unk0: u64) -> Result<::nn::am::service::ILibraryAppletCreator> {
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
