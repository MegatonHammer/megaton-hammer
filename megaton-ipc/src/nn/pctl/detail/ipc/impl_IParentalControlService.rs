
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IParentalControlService(Session);

impl IParentalControlService {
	pub fn CheckFreeCommunicationPermission(&self, ) -> Result<()> {
		let req = Request::new(1001)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ConfirmLaunchApplicationPermission(&self, unk0: bool, unk1: ::nn::ncm::ApplicationId, unk2: &[i8]) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::ncm::ApplicationId,
		}
		let req = Request::new(1002)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ConfirmResumeApplicationPermission(&self, unk0: bool, unk1: ::nn::ncm::ApplicationId, unk2: &[i8]) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::ncm::ApplicationId,
		}
		let req = Request::new(1003)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ConfirmSnsPostPermission(&self, ) -> Result<()> {
		let req = Request::new(1004)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ConfirmSystemSettingsPermission(&self, ) -> Result<()> {
		let req = Request::new(1005)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsRestrictionTemporaryUnlocked(&self, ) -> Result<bool> {
		let req = Request::new(1006)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn RevertRestrictionTemporaryUnlocked(&self, ) -> Result<()> {
		let req = Request::new(1007)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn EnterRestrictedSystemSettings(&self, ) -> Result<()> {
		let req = Request::new(1008)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn LeaveRestrictedSystemSettings(&self, ) -> Result<()> {
		let req = Request::new(1009)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsRestrictedSystemSettingsEntered(&self, ) -> Result<bool> {
		let req = Request::new(1010)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn RevertRestrictedSystemSettingsEntered(&self, ) -> Result<()> {
		let req = Request::new(1011)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetRestrictedFeatures(&self, ) -> Result<i32> {
		let req = Request::new(1012)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn IsRestrictionEnabled(&self, ) -> Result<bool> {
		let req = Request::new(1031)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetSafetyLevel(&self, ) -> Result<i32> {
		let req = Request::new(1032)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetSafetyLevel(&self, unk0: i32) -> Result<()> {
		let req = Request::new(1033)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetSafetyLevelSettings(&self, unk0: i32) -> Result<::nn::pctl::SafetyLevelSettings> {
		let req = Request::new(1034)
			.args(unk0)
			;
		let mut res : Response<::nn::pctl::SafetyLevelSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetCurrentSettings(&self, ) -> Result<::nn::pctl::SafetyLevelSettings> {
		let req = Request::new(1035)
			.args(())
			;
		let mut res : Response<::nn::pctl::SafetyLevelSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetCustomSafetyLevelSettings(&self, unk0: ::nn::pctl::SafetyLevelSettings) -> Result<()> {
		let req = Request::new(1036)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDefaultRatingOrganization(&self, ) -> Result<i32> {
		let req = Request::new(1037)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn SetDefaultRatingOrganization(&self, unk0: i32) -> Result<()> {
		let req = Request::new(1038)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetFreeCommunicationApplicationListCount(&self, ) -> Result<i32> {
		let req = Request::new(1039)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn AddToFreeCommunicationApplicationList(&self, unk0: ::nn::ncm::ApplicationId) -> Result<()> {
		let req = Request::new(1042)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DeleteSettings(&self, ) -> Result<()> {
		let req = Request::new(1043)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetFreeCommunicationApplicationList(&self, unk0: i32, unk2: &mut [::nn::pctl::FreeCommunicationApplicationInfo]) -> Result<i32> {
		let req = Request::new(1044)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn UpdateFreeCommunicationApplicationList(&self, unk0: &[::nn::pctl::FreeCommunicationApplicationInfo]) -> Result<()> {
		let req = Request::new(1045)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DisableFeaturesForReset(&self, ) -> Result<()> {
		let req = Request::new(1046)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn NotifyApplicationDownloadStarted(&self, unk0: ::nn::ncm::ApplicationId) -> Result<()> {
		let req = Request::new(1047)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn UnlockRestrictionTemporarily(&self, unk0: &[i8]) -> Result<()> {
		let req = Request::new(1201)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn UnlockSystemSettingsRestriction(&self, unk0: &[i8]) -> Result<()> {
		let req = Request::new(1202)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetPinCode(&self, unk0: &[i8]) -> Result<()> {
		let req = Request::new(1203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GenerateInquiryCode(&self, ) -> Result<::nn::pctl::InquiryCode> {
		let req = Request::new(1204)
			.args(())
			;
		let mut res : Response<::nn::pctl::InquiryCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn CheckMasterKey(&self, unk0: ::nn::pctl::InquiryCode, unk1: &[i8]) -> Result<bool> {
		let req = Request::new(1205)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetPinCodeLength(&self, ) -> Result<i32> {
		let req = Request::new(1206)
			.args(())
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetPinCodeChangedEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn IsPairingActive(&self, ) -> Result<bool> {
		let req = Request::new(1403)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetSettingsLastUpdated(&self, ) -> Result<::nn::time::PosixTime> {
		let req = Request::new(1406)
			.args(())
			;
		let mut res : Response<::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetPairingAccountInfo(&self, unk0: ::nn::pctl::detail::PairingInfoBase) -> Result<::nn::pctl::detail::PairingAccountInfoBase> {
		let req = Request::new(1411)
			.args(unk0)
			;
		let mut res : Response<::nn::pctl::detail::PairingAccountInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetAccountNickname(&self, unk0: ::nn::pctl::detail::PairingAccountInfoBase, unk2: &mut [i8]) -> Result<u32> {
		let req = Request::new(1421)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetAccountState(&self, unk0: ::nn::pctl::detail::PairingAccountInfoBase) -> Result<i32> {
		let req = Request::new(1424)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetSynchronizationEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn StartPlayTimer(&self, ) -> Result<()> {
		let req = Request::new(1451)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn StopPlayTimer(&self, ) -> Result<()> {
		let req = Request::new(1452)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn IsPlayTimerEnabled(&self, ) -> Result<bool> {
		let req = Request::new(1453)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetPlayTimerRemainingTime(&self, ) -> Result<::nn::TimeSpanType> {
		let req = Request::new(1454)
			.args(())
			;
		let mut res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn IsRestrictedByPlayTimer(&self, ) -> Result<bool> {
		let req = Request::new(1455)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetPlayTimerSettings(&self, ) -> Result<::nn::pctl::PlayTimerSettings> {
		let req = Request::new(1456)
			.args(())
			;
		let mut res : Response<::nn::pctl::PlayTimerSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetPlayTimerEventToRequestSuspension(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn NotifyWrongPinCodeInputManyTimes(&self, ) -> Result<()> {
		let req = Request::new(1471)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn CancelNetworkRequest(&self, ) -> Result<()> {
		let req = Request::new(1472)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetUnlinkedEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ClearUnlinkedEvent(&self, ) -> Result<()> {
		let req = Request::new(1474)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DisableAllFeatures(&self, ) -> Result<bool> {
		let req = Request::new(1601)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn PostEnableAllFeatures(&self, ) -> Result<bool> {
		let req = Request::new(1602)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn IsAllFeaturesDisabled(&self, ) -> Result<(bool, bool)> {
		let req = Request::new(1603)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: bool,
			unk1: bool,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}
	pub fn DeleteFromFreeCommunicationApplicationListForDebug(&self, unk0: ::nn::ncm::ApplicationId) -> Result<()> {
		let req = Request::new(1901)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn ClearFreeCommunicationApplicationListForDebug(&self, ) -> Result<()> {
		let req = Request::new(1902)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn DeletePairing(&self, ) -> Result<()> {
		let req = Request::new(1941)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetPlayTimerSettingsForDebug(&self, unk0: ::nn::pctl::PlayTimerSettings) -> Result<()> {
		let req = Request::new(1951)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetPlayTimerSpentTimeForTest(&self, ) -> Result<::nn::TimeSpanType> {
		let req = Request::new(1952)
			.args(())
			;
		let mut res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn RequestPairingAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn FinishRequestPairing(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::pctl::detail::PairingInfoBase> {
		let req = Request::new(2002)
			.args(unk0)
			;
		let mut res : Response<::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn AuthorizePairingAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn FinishAuthorizePairing(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::pctl::detail::PairingInfoBase> {
		let req = Request::new(2004)
			.args(unk0)
			;
		let mut res : Response<::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn RetrievePairingInfoAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn FinishRetrievePairingInfo(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::pctl::detail::PairingInfoBase> {
		let req = Request::new(2006)
			.args(unk0)
			;
		let mut res : Response<::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn UnlinkPairingAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn FinishUnlinkPairing(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetAccountMiiImageAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn FinishGetAccountMiiImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetAccountMiiImageContentTypeAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn FinishGetAccountMiiImageContentType(&self, unk0: ::nn::pctl::detail::AsyncData, unk2: &mut [i8]) -> Result<u32> {
		let req = Request::new(2012)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn SynchronizeParentalControlSettingsAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn FinishSynchronizeParentalControlSettings(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<()> {
		let req = Request::new(2014)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn FinishSynchronizeParentalControlSettingsWithLastUpdated(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::time::PosixTime> {
		let req = Request::new(2015)
			.args(unk0)
			;
		let mut res : Response<::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for IParentalControlService {
	unsafe fn from_kobject(obj: KObject) -> IParentalControlService {
		IParentalControlService(Session::from_kobject(obj))
	}
}
