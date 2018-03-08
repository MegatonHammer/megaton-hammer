
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct INcmInterface(Session);

impl INcmInterface {
	pub fn Unknown2(&self, ) -> Result<(u64)> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown3(&self, ) -> Result<(u64)> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown4(&self, ) -> Result<(::nn::ncm::detail::INcmInterface4Unknown)> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown5(&self, ) -> Result<(::nn::ncm::detail::INcmInterface5Unknown)> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown9(&self, ) -> Result<(u64)> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown11(&self, ) -> Result<(u64)> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for INcmInterface {
	unsafe fn from_kobject(obj: KObject) -> INcmInterface {
		INcmInterface(Session::from_kobject(obj))
	}
}
