
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct INotificationService(Session);

impl INotificationService {
	// fn GetEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Clear(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Pop(&self, ) -> Result<::nn::friends::detail::ipc::SizedNotificationInfo> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::friends::detail::ipc::SizedNotificationInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for INotificationService {
	unsafe fn from_kobject(obj: KObject) -> INotificationService {
		INotificationService(Session::from_kobject(obj))
	}
}
