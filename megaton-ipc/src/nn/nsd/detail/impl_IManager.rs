
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IManager(Session);

impl IManager {
	pub fn GetSettingName(&self, unk0: [u8; 0x100]) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetEnvironmentIdentifier(&self, unk0: [u8; 8]) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDeviceId(&self, ) -> Result<u128> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn DeleteSettings(&self, unk0: u32) -> Result<()> {
		let req = Request::new(13)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn ImportSettings(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Resolve(&self, unk0: [u8; 0x100], unk1: [u8; 0x100]) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ResolveEx(&self, unk0: [u8; 0x100], unk2: [u8; 0x100]) -> Result<u32> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetNasServiceSetting(&self, unk0: [u8; 0x10], unk1: [u8; 0x108]) -> Result<()> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetNasServiceSettingEx(&self, unk0: [u8; 0x10], unk2: [u8; 0x108]) -> Result<u32> {
		let req = Request::new(31)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetNasRequestFqdn(&self, unk0: [u8; 0x100]) -> Result<()> {
		let req = Request::new(40)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetNasRequestFqdnEx(&self, unk1: [u8; 0x100]) -> Result<u32> {
		let req = Request::new(41)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetNasApiFqdn(&self, unk0: [u8; 0x100]) -> Result<()> {
		let req = Request::new(42)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetNasApiFqdnEx(&self, unk1: [u8; 0x100]) -> Result<u32> {
		let req = Request::new(43)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetCurrentSetting(&self, unk0: [u8; 0x12bf0]) -> Result<()> {
		let req = Request::new(50)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ReadSaveDataFromFsForTest(&self, unk0: [u8; 0x12bf0]) -> Result<()> {
		let req = Request::new(60)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn WriteSaveDataToFsForTest(&self, unk0: [u8; 0x12bf0]) -> Result<()> {
		let req = Request::new(61)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DeleteSaveDataOfFsForTest(&self, ) -> Result<()> {
		let req = Request::new(62)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IManager {
	unsafe fn from_kobject(obj: KObject) -> IManager {
		IManager(Session::from_kobject(obj))
	}
}
