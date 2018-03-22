
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDirectory(Session);

impl AsRef<Session> for IDirectory {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDirectory {
	pub fn read(&self, unk1: &mut [::nn::fssrv::sf::IDirectoryEntry]) -> Result<u64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_entry_count(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDirectory {
	unsafe fn from_kobject(obj: KObject) -> IDirectory {
		IDirectory(Session::from_kobject(obj))
	}
}
