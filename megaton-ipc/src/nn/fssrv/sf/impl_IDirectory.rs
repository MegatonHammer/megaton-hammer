
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IDirectory(Session);

impl IDirectory {
	pub fn Read(&self, unk1: &mut Option<::nn::fssrv::sf::IDirectoryEntry>) -> Result<u64> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetEntryCount(&self, ) -> Result<u64> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IDirectory {
	unsafe fn from_kobject(obj: KObject) -> IDirectory {
		IDirectory(Session::from_kobject(obj))
	}
}
