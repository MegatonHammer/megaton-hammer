
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IHomeMenuFunctions(Session);

impl IHomeMenuFunctions {
	pub fn RequestToGetForeground(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn LockForeground(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnlockForeground(&self, ) -> Result<()> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PopFromGeneralChannel(&self, ) -> Result<(::nn::am::service::IStorage)> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetPopFromGeneralChannelEvent(&self, ) -> Result<(KObject)> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetHomeButtonWriterLockAccessor(&self, ) -> Result<(::nn::am::service::ILockAccessor)> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetWriterLockAccessorEx(&self, unk0: i32) -> Result<(::nn::am::service::ILockAccessor)> {
		let req = Request::new(31)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IHomeMenuFunctions {
	unsafe fn from_kobject(obj: KObject) -> IHomeMenuFunctions {
		IHomeMenuFunctions(Session::from_kobject(obj))
	}
}
