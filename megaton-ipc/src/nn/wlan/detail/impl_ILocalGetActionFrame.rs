
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ILocalGetActionFrame(Session);

impl ILocalGetActionFrame {
	pub fn Unknown0(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ILocalGetActionFrame {
	unsafe fn from_kobject(obj: KObject) -> ILocalGetActionFrame {
		ILocalGetActionFrame(Session::from_kobject(obj))
	}
}
