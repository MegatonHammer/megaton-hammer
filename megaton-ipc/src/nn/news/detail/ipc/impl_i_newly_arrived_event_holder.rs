
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INewlyArrivedEventHolder(Session);

impl AsRef<Session> for INewlyArrivedEventHolder {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INewlyArrivedEventHolder {
	pub fn unknown0(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for INewlyArrivedEventHolder {
	unsafe fn from_kobject(obj: KObject) -> INewlyArrivedEventHolder {
		INewlyArrivedEventHolder(Session::from_kobject(obj))
	}
}
