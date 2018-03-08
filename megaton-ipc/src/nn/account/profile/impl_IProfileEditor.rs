
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IProfileEditor(Session);

impl IProfileEditor {
	pub fn Get(&self, unk1: &mut Option<::nn::account::profile::UserData>) -> Result<(::nn::account::profile::ProfileBase)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetBase(&self, ) -> Result<(::nn::account::profile::ProfileBase)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::account::profile::ProfileBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetImageSize(&self, ) -> Result<(u32)> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn LoadImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Store(&self, unk0: ::nn::account::profile::ProfileBase, unk1: &::nn::account::profile::UserData) -> Result<()> {
		let req = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn StoreWithImage(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IProfileEditor {
	unsafe fn from_kobject(obj: KObject) -> IProfileEditor {
		IProfileEditor(Session::from_kobject(obj))
	}
}
