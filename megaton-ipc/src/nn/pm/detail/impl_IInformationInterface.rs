
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IInformationInterface(Session);

impl IInformationInterface {
	pub fn GetTitleId(&self, unk0: u64) -> Result<(u64)> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IInformationInterface {
	unsafe fn from_kobject(obj: KObject) -> IInformationInterface {
		IInformationInterface(Session::from_kobject(obj))
	}
}
