
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IParentalControlServiceFactory(Session);

impl IParentalControlServiceFactory {
	pub fn GetService(&self, unk0: u64) -> Result<::nn::pctl::detail::ipc::IParentalControlService> {
		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
}

impl FromKObject for IParentalControlServiceFactory {
	unsafe fn from_kobject(obj: KObject) -> IParentalControlServiceFactory {
		IParentalControlServiceFactory(Session::from_kobject(obj))
	}
}
