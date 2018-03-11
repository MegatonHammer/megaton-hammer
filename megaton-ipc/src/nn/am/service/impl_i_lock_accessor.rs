
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILockAccessor(Session);

impl AsRef<Session> for ILockAccessor {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILockAccessor {
	pub fn try_lock(&self, unk0: bool) -> Result<(bool, KObject)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn unlock(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for ILockAccessor {
	unsafe fn from_kobject(obj: KObject) -> ILockAccessor {
		ILockAccessor(Session::from_kobject(obj))
	}
}
