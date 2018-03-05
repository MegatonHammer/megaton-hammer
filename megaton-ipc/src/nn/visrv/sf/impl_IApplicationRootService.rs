
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IApplicationRootService(Session);

impl IApplicationRootService {
	pub fn GetDisplayService(&self, unk0: u32) -> Result<::nn::visrv::sf::IApplicationDisplayService> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
}

impl FromKObject for IApplicationRootService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationRootService {
		IApplicationRootService(Session::from_kobject(obj))
	}
}
