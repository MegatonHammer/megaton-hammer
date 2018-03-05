
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ILockAccessor(Session);

impl ILockAccessor {
	// fn TryLock(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unlock(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ILockAccessor {
	unsafe fn from_kobject(obj: KObject) -> ILockAccessor {
		ILockAccessor(Session::from_kobject(obj))
	}
}
