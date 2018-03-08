
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAdministrator(Session);

impl IAdministrator {
	pub fn CheckAvailability(&self, ) -> Result<()> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAccountId(&self, ) -> Result<(::nn::account::NetworkServiceAccountId)> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EnsureIdTokenCacheAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn LoadIdTokenCache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetSystemProgramIdentification(&self, unk0: u64, unk2: &::nn::account::SystemProgramIdentification) -> Result<()> {
		let req = Request::new(100)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetNintendoAccountId(&self, ) -> Result<(::nn::account::NintendoAccountId)> {
		let req = Request::new(120)
			.args(())
			;
		let mut res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetNintendoAccountUserResourceCache(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RefreshNintendoAccountUserResourceCacheAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(131)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn RefreshNintendoAccountUserResourceCacheAsyncIfSecondsElapsed(&self, unk0: u32) -> Result<(bool, ::nn::account::detail::IAsyncContext)> {
		let req = Request::new(132)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

	pub fn CreateAuthorizationRequest(&self, unk0: u32, unk1: KObject, unk2: &::nn::account::nas::NasClientInfo, unk3: &::nn::account::NintendoAccountAuthorizationRequestParameters) -> Result<(::nn::account::nas::IAuthorizationRequest)> {
		let req = Request::new(150)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn IsRegistered(&self, ) -> Result<(bool)> {
		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn RegisterAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(201)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn UnregisterAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(202)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn DeleteRegistrationInfoLocally(&self, ) -> Result<()> {
		let req = Request::new(203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SynchronizeProfileAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(220)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn UploadProfileAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(221)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn SynchronizeProfileAsyncIfSecondsElapsed(&self, unk0: u32) -> Result<(bool, ::nn::account::detail::IAsyncContext)> {
		let req = Request::new(222)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

	pub fn IsLinkedWithNintendoAccount(&self, ) -> Result<(bool)> {
		let req = Request::new(250)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CreateProcedureToLinkWithNintendoAccount(&self, ) -> Result<(::nn::account::nas::IOAuthProcedureForNintendoAccountLinkage)> {
		let req = Request::new(251)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn ResumeProcedureToLinkWithNintendoAccount(&self, unk0: ::nn::account::detail::Uuid) -> Result<(::nn::account::nas::IOAuthProcedureForNintendoAccountLinkage)> {
		let req = Request::new(252)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateProcedureToUpdateLinkageStateOfNintendoAccount(&self, ) -> Result<(::nn::account::http::IOAuthProcedure)> {
		let req = Request::new(255)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn ResumeProcedureToUpdateLinkageStateOfNintendoAccount(&self, unk0: ::nn::account::detail::Uuid) -> Result<(::nn::account::http::IOAuthProcedure)> {
		let req = Request::new(256)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateProcedureToLinkNnidWithNintendoAccount(&self, ) -> Result<(::nn::account::http::IOAuthProcedure)> {
		let req = Request::new(260)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn ResumeProcedureToLinkNnidWithNintendoAccount(&self, unk0: ::nn::account::detail::Uuid) -> Result<(::nn::account::http::IOAuthProcedure)> {
		let req = Request::new(261)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn ProxyProcedureToAcquireApplicationAuthorizationForNintendoAccount(&self, unk0: ::nn::account::detail::Uuid) -> Result<(::nn::account::http::IOAuthProcedure)> {
		let req = Request::new(280)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn DebugUnlinkNintendoAccountAsync(&self, ) -> Result<(::nn::account::detail::IAsyncContext)> {
		let req = Request::new(997)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn DebugSetAvailabilityErrorDetail(&self, unk0: u32) -> Result<()> {
		let req = Request::new(998)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAdministrator {
	unsafe fn from_kobject(obj: KObject) -> IAdministrator {
		IAdministrator(Session::from_kobject(obj))
	}
}
