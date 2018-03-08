
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IApplicationProxyService(Session);

impl IApplicationProxyService {
	pub fn OpenApplicationProxy(&self, unk0: u64, unk2: KObject) -> Result<(::nn::am::service::IApplicationProxy)> {
		let req = Request::new(0)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IApplicationProxyService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationProxyService {
		IApplicationProxyService(Session::from_kobject(obj))
	}
}
