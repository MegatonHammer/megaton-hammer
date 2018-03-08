
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IMonitorService(Session);

impl IMonitorService {
	pub fn GetNifmStatus(&self, ) -> Result<(u32)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown1(&self, unk0: [u8; 0x480]) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2(&self, ) -> Result<(u32, u32)> {
		let req = Request::new(2)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn Unknown3(&self, ) -> Result<(u16)> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u16> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn Unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn StartMonitor(&self, ) -> Result<()> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopMonitor(&self, ) -> Result<()> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IMonitorService {
	unsafe fn from_kobject(obj: KObject) -> IMonitorService {
		IMonitorService(Session::from_kobject(obj))
	}
}
