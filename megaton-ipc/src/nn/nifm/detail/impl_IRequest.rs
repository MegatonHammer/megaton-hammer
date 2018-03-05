
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IRequest(Session);

impl IRequest {
	pub fn GetRequestState(&self, ) -> Result<i32> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetResult(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetSystemEventReadableHandles(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Cancel(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn Submit(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetRequirement(&self, unk0: ::nn::nifm::Requirement) -> Result<()> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetRequirementPreset(&self, unk0: i32) -> Result<()> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetPriority(&self, unk0: u8) -> Result<()> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetNetworkProfileId(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetRejectable(&self, unk0: bool) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetConnectionConfirmationOption(&self, unk0: i8) -> Result<()> {
		let req = Request::new(11)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetPersistent(&self, unk0: bool) -> Result<()> {
		let req = Request::new(12)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetInstant(&self, unk0: bool) -> Result<()> {
		let req = Request::new(13)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetSustainable(&self, unk0: bool, unk1: u8) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u8,
		}
		let req = Request::new(14)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetRawPriority(&self, unk0: u8) -> Result<()> {
		let req = Request::new(15)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetGreedy(&self, unk0: bool) -> Result<()> {
		let req = Request::new(16)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetSharable(&self, unk0: bool) -> Result<()> {
		let req = Request::new(17)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetRequirementByRevision(&self, unk0: u32) -> Result<()> {
		let req = Request::new(18)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetRequirement(&self, ) -> Result<::nn::nifm::Requirement> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<::nn::nifm::Requirement> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetRevision(&self, ) -> Result<u32> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetAppletInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetAdditionalInfo(&self, unk1: &mut Option<::nn::nifm::AdditionalInfo>) -> Result<u32> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetKeptInSleep(&self, unk0: bool) -> Result<()> {
		let req = Request::new(23)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn RegisterSocketDescriptor(&self, unk0: i32) -> Result<()> {
		let req = Request::new(24)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn UnregisterSocketDescriptor(&self, unk0: i32) -> Result<()> {
		let req = Request::new(25)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IRequest {
	unsafe fn from_kobject(obj: KObject) -> IRequest {
		IRequest(Session::from_kobject(obj))
	}
}
