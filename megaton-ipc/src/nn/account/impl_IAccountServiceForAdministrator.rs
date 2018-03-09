
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAccountServiceForAdministrator(Session);

impl IAccountServiceForAdministrator {
	pub fn get_service() -> Result<IAccountServiceForAdministrator> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"acc:su\0\0").map(|s| unsafe { IAccountServiceForAdministrator::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAccountServiceForAdministrator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAccountServiceForAdministrator {
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

	pub fn GetUserRegistrationNotifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetUserStateChangeNotifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetBaasAccountManagerForSystemService(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::baas::IManagerForSystemService> {
		let req = Request::new(102)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetBaasUserAvailabilityChangeNotifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		let req = Request::new(103)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetProfileUpdateNotifier(&self, ) -> Result<::nn::account::detail::INotifier> {
		let req = Request::new(104)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn StoreSaveDataThumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearSaveDataThumbnail(&self, unk0: ::nn::account::Uid, unk1: ::nn::ApplicationId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::ApplicationId,
		}
		let req = Request::new(111)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn LoadSaveDataThumbnail(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetUserLastOpenedApplication(&self, unk0: ::nn::account::Uid) -> Result<(u32, ::nn::ApplicationId)> {
		let req = Request::new(190)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: ::nn::ApplicationId,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn BeginUserRegistration(&self, ) -> Result<::nn::account::Uid> {
		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<::nn::account::Uid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CompleteUserRegistration(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(201)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CancelUserRegistration(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(202)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteUser(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(203)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetUserPosition(&self, unk0: i32, unk1: ::nn::account::Uid) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(204)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetProfileEditor(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::profile::IProfileEditor> {
		let req = Request::new(205)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CompleteUserRegistrationForcibly(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(206)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateFloatingRegistrationRequest(&self, unk0: u32, unk1: &KObject) -> Result<::nn::account::baas::IFloatingRegistrationRequest> {
		let req = Request::new(210)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn AuthenticateServiceAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(230)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetBaasAccountAdministrator(&self, unk0: ::nn::account::Uid) -> Result<::nn::account::baas::IAdministrator> {
		let req = Request::new(250)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn ProxyProcedureForGuestLoginWithNintendoAccount(&self, unk0: ::nn::account::detail::Uuid) -> Result<::nn::account::nas::IOAuthProcedureForExternalNsa> {
		let req = Request::new(290)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn ProxyProcedureForFloatingRegistrationWithNintendoAccount(&self, unk0: ::nn::account::detail::Uuid) -> Result<::nn::account::nas::IOAuthProcedureForExternalNsa> {
		let req = Request::new(291)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn SuspendBackgroundDaemon(&self, ) -> Result<::nn::account::detail::ISessionObject> {
		let req = Request::new(299)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn DebugInvalidateTokenCacheForUser(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(997)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DebugSetUserStateClose(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(998)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DebugSetUserStateOpen(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(999)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAccountServiceForAdministrator {
	unsafe fn from_kobject(obj: KObject) -> IAccountServiceForAdministrator {
		IAccountServiceForAdministrator(Session::from_kobject(obj))
	}
}
