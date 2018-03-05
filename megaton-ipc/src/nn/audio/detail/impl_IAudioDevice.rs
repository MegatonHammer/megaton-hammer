
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAudioDevice(Session);

impl IAudioDevice {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown5(&self, ) -> Result<u32> {
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
	// fn Unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioDevice {
	unsafe fn from_kobject(obj: KObject) -> IAudioDevice {
		IAudioDevice(Session::from_kobject(obj))
	}
}
