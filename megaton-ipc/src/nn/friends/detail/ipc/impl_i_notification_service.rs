
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INotificationService(Session);

impl AsRef<Session> for INotificationService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INotificationService {
	pub fn get_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn clear(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn pop(&self, ) -> Result<::nn::friends::detail::ipc::SizedNotificationInfo> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::friends::detail::ipc::SizedNotificationInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for INotificationService {
	unsafe fn from_kobject(obj: KObject) -> INotificationService {
		INotificationService(Session::from_kobject(obj))
	}
}
