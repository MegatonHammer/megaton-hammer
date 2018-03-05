
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IManagerPrivileged(Session);

impl IManagerPrivileged {
	pub fn OpenSession(&self, ) -> Result<::nn::apm::ISession> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
}

impl FromKObject for IManagerPrivileged {
	unsafe fn from_kobject(obj: KObject) -> IManagerPrivileged {
		IManagerPrivileged(Session::from_kobject(obj))
	}
}
