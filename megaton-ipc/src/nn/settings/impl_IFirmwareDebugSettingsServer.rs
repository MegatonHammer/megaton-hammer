
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IFirmwareDebugSettingsServer(Session);

impl IFirmwareDebugSettingsServer {
	pub fn get_service() -> Result<IFirmwareDebugSettingsServer> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"set:fd\0\0").map(|s| unsafe { IFirmwareDebugSettingsServer::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IFirmwareDebugSettingsServer {
	// fn SetSettingsItemValue(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ResetSettingsItemValue(&self, unk0: &::nn::settings::SettingsName, unk1: &::nn::settings::SettingsItemKey) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateSettingsItemKeyIterator(&self, unk0: &::nn::settings::SettingsName) -> Result<::nn::settings::ISettingsItemKeyIterator> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn ReadSettings(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn ResetSettings(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn SetWebInspectorFlag(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn SetAllowedSslHosts(&self, ) -> Result<()> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn SetHostFsMountPoint(&self, ) -> Result<()> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFirmwareDebugSettingsServer {
	unsafe fn from_kobject(obj: KObject) -> IFirmwareDebugSettingsServer {
		IFirmwareDebugSettingsServer(Session::from_kobject(obj))
	}
}
