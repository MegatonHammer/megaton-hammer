
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IReceiverService(Session);

impl IReceiverService {
	pub fn Unknown0(&self, ) -> Result<(Session)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IReceiverService {
	unsafe fn from_kobject(obj: KObject) -> IReceiverService {
		IReceiverService(Session::from_kobject(obj))
	}
}
