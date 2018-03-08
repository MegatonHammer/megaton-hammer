
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioDevice(Session);

impl IAudioDevice {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown4(&self, ) -> Result<(KObject)> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown5(&self, ) -> Result<(u32)> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn Unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown11(&self, ) -> Result<(KObject)> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown12(&self, ) -> Result<(KObject)> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IAudioDevice {
	unsafe fn from_kobject(obj: KObject) -> IAudioDevice {
		IAudioDevice(Session::from_kobject(obj))
	}
}
