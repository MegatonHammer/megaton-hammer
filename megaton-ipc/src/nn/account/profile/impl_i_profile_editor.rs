
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IProfileEditor(Session);

impl AsRef<Session> for IProfileEditor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IProfileEditor {
	pub fn get(&self, unk1: &mut ::nn::account::profile::UserData) -> Result<::nn::account::profile::ProfileBase> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_base(&self, ) -> Result<::nn::account::profile::ProfileBase> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_image_size(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn load_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn store(&self, unk0: ::nn::account::profile::ProfileBase, unk1: &::nn::account::profile::UserData) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(unk1, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn store_with_image(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IProfileEditor {
	unsafe fn from_kobject(obj: KObject) -> IProfileEditor {
		IProfileEditor(Session::from_kobject(obj))
	}
}
