
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ILocalGetFrame(Session);

impl ILocalGetFrame {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for ILocalGetFrame {
	unsafe fn from_kobject(obj: KObject) -> ILocalGetFrame {
		ILocalGetFrame(Session::from_kobject(obj))
	}
}
