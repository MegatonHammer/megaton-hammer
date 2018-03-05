
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IPadSession(Session);

impl IPadSession {
	pub fn SetDirection(&self, unk0: u32) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDirection(&self, ) -> Result<u32> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetInterruptMode(&self, unk0: u32) -> Result<()> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetInterruptMode(&self, ) -> Result<u32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetInterruptEnable(&self, unk0: u8) -> Result<()> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetInterruptEnable(&self, ) -> Result<u8> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetInterruptStatus(&self, ) -> Result<u32> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn ClearInterruptStatus(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetValue(&self, unk0: u32) -> Result<()> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetValue(&self, ) -> Result<u32> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn BindInterrupt(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn UnbindInterrupt(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetDebounceEnabled(&self, unk0: u8) -> Result<()> {
		let req = Request::new(12)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDebounceEnabled(&self, ) -> Result<u8> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetDebounceTime(&self, unk0: u32) -> Result<()> {
		let req = Request::new(14)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDebounceTime(&self, ) -> Result<u32> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IPadSession {
	unsafe fn from_kobject(obj: KObject) -> IPadSession {
		IPadSession(Session::from_kobject(obj))
	}
}
