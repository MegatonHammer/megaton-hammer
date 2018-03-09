
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IManager(Session);

impl IManager {
	pub fn get_service() -> Result<IManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"nsd:a\0\0\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"nsd:u\0\0\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManager {
	// fn GetSettingName(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetEnvironmentIdentifier(&self, UNKNOWN) -> Result<UNKNOWN>;
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
	// fn Resolve(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ResolveEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNasServiceSetting(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNasServiceSettingEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNasRequestFqdn(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNasRequestFqdnEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNasApiFqdn(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNasApiFqdnEx(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetCurrentSetting(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ReadSaveDataFromFsForTest(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn WriteSaveDataToFsForTest(&self, UNKNOWN) -> Result<UNKNOWN>;
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
