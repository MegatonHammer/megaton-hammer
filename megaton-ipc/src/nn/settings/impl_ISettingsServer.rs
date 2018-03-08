
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISettingsServer(Session);

impl ISettingsServer {
	pub fn GetLanguageCode(&self, ) -> Result<(::nn::settings::LanguageCode)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::settings::LanguageCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAvailableLanguageCodes(&self, unk1: &mut [::nn::settings::LanguageCode]) -> Result<(i32)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn MakeLanguageCode(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAvailableLanguageCodeCount(&self, ) -> Result<(i32)> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetRegionCode(&self, ) -> Result<(i32)> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn GetAvailableLanguageCodes2(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn GetAvailableLanguageCodeCount2(&self, ) -> Result<()> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn GetKeyCodeMap(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISettingsServer {
	unsafe fn from_kobject(obj: KObject) -> ISettingsServer {
		ISettingsServer(Session::from_kobject(obj))
	}
}
