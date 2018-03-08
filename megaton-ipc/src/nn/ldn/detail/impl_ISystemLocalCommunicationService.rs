
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISystemLocalCommunicationService(Session);

impl ISystemLocalCommunicationService {
	pub fn Unknown0(&self, ) -> Result<(u32)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetNetworkInfo(&self, unk0: [u8; 0x480]) -> Result<()> {
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
	pub fn GetUnkWaitEvent(&self, ) -> Result<(KObject)> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn Unknown101(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown102(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown103(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn OpenAccessPoint(&self, ) -> Result<()> {
		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CloseAccessPoint(&self, ) -> Result<()> {
		let req = Request::new(201)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown202(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown203(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn DestroyNetwork(&self, ) -> Result<()> {
		let req = Request::new(204)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown205(&self, unk0: u32) -> Result<()> {
		let req = Request::new(205)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown206(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown207(&self, unk0: u8) -> Result<()> {
		let req = Request::new(207)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown208(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown209(&self, ) -> Result<()> {
		let req = Request::new(209)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn OpenStation(&self, ) -> Result<()> {
		let req = Request::new(300)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CloseStation(&self, ) -> Result<()> {
		let req = Request::new(301)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown302(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown303(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Disconnect(&self, ) -> Result<()> {
		let req = Request::new(304)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn InitializeSystem(&self, unk0: u64) -> Result<()> {
		let req = Request::new(400)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn TerminateSystem(&self, ) -> Result<()> {
		let req = Request::new(401)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISystemLocalCommunicationService {
	unsafe fn from_kobject(obj: KObject) -> ISystemLocalCommunicationService {
		ISystemLocalCommunicationService(Session::from_kobject(obj))
	}
}
