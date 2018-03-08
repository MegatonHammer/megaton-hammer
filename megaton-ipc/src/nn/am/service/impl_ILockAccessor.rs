
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ILockAccessor(Session);

impl ILockAccessor {
	pub fn TryLock(&self, unk0: bool) -> Result<(bool, KObject)> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn Unlock(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetEvent(&self, ) -> Result<(KObject)> {
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
