
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IAccountServiceForApplication(Session);

impl IAccountServiceForApplication {
	pub fn get_service() -> Result<IAccountServiceForApplication> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"acc:u0\0\0").map(|s| unsafe { IAccountServiceForApplication::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAccountServiceForApplication {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAccountServiceForApplication {
	pub fn GetUserCount(&self, ) -> Result<i32> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetUserExistence(&self, unk0: ::nn::account::Uid) -> Result<bool> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn ListAllUsers(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ListOpenUsers(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetLastOpenedUser(&self, ) -> Result<::nn::account::Uid> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetProfile(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::profile::IProfile> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetProfileDigest(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::ProfileDigest> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<::nn::account::ProfileDigest> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn IsUserRegistrationRequestPermitted(&self, unk0: u64) -> Result<bool> {
		let req = Request::new(50)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn TrySelectUserWithoutInteraction(&self, unk0: bool) -> Result<::nn::account::Uid> {
		let req = Request::new(51)
			.args(unk0)
			;
		let mut res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn InitializeApplicationInfo(&self, unk0: u64) -> Result<()> {
		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetBaasAccountManagerForApplication(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::baas::IManagerForApplication> {
		let req = Request::new(101)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn AuthenticateApplicationAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn StoreSaveDataThumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearSaveDataThumbnail(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(111)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateGuestLoginRequest(&self, unk0: u32, unk1: &KObject) -> Result<::nn::account::baas::IGuestLoginRequest> {
		let req = Request::new(120)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IAccountServiceForApplication {
	unsafe fn from_kobject(obj: KObject) -> IAccountServiceForApplication {
		IAccountServiceForApplication(Session::from_kobject(obj))
	}
}
