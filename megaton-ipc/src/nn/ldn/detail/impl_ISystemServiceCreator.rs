
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISystemServiceCreator(Session);

impl ISystemServiceCreator {
	pub fn GetSystemLocalCommunicationService(&self, ) -> Result<Session> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
}

impl FromKObject for ISystemServiceCreator {
	unsafe fn from_kobject(obj: KObject) -> ISystemServiceCreator {
		ISystemServiceCreator(Session::from_kobject(obj))
	}
}
