
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISocketGetFrame(Session);

impl ISocketGetFrame {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for ISocketGetFrame {
	unsafe fn from_kobject(obj: KObject) -> ISocketGetFrame {
		ISocketGetFrame(Session::from_kobject(obj))
	}
}
