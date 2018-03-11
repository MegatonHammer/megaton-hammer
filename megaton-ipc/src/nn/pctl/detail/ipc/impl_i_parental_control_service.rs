
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IParentalControlService(Session);

impl AsRef<Session> for IParentalControlService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IParentalControlService {
	pub fn check_free_communication_permission(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1001)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn confirm_launch_application_permission(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn confirm_resume_application_permission(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn confirm_sns_post_permission(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1004)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn confirm_system_settings_permission(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1005)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_restriction_temporary_unlocked(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1006)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn revert_restriction_temporary_unlocked(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1007)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enter_restricted_system_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1008)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn leave_restricted_system_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1009)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_restricted_system_settings_entered(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1010)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn revert_restricted_system_settings_entered(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1011)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_restricted_features(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1012)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_restriction_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1031)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_safety_level(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1032)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_safety_level(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1033)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_safety_level_settings(&self, unk0: i32) -> Result<::nn::pctl::SafetyLevelSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1034)
			.args(unk0)
			;
		let res : Response<::nn::pctl::SafetyLevelSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_settings(&self, ) -> Result<::nn::pctl::SafetyLevelSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1035)
			.args(())
			;
		let res : Response<::nn::pctl::SafetyLevelSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_custom_safety_level_settings(&self, unk0: ::nn::pctl::SafetyLevelSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1036)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_default_rating_organization(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1037)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_default_rating_organization(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1038)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_free_communication_application_list_count(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1039)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn add_to_free_communication_application_list(&self, unk0: ::nn::ncm::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1042)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_settings(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1043)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_free_communication_application_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn update_free_communication_application_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn disable_features_for_reset(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1046)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn notify_application_download_started(&self, unk0: ::nn::ncm::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1047)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unlock_restriction_temporarily(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unlock_system_settings_restriction(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_pin_code(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn generate_inquiry_code(&self, ) -> Result<::nn::pctl::InquiryCode> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1204)
			.args(())
			;
		let res : Response<::nn::pctl::InquiryCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn check_master_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_pin_code_length(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1206)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_pin_code_changed_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1207)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn is_pairing_active(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1403)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_settings_last_updated(&self, ) -> Result<::nn::time::PosixTime> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1406)
			.args(())
			;
		let res : Response<::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_pairing_account_info(&self, unk0: ::nn::pctl::detail::PairingInfoBase) -> Result<::nn::pctl::detail::PairingAccountInfoBase> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1411)
			.args(unk0)
			;
		let res : Response<::nn::pctl::detail::PairingAccountInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_account_nickname(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_account_state(&self, unk0: ::nn::pctl::detail::PairingAccountInfoBase) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1424)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_synchronization_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1432)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn start_play_timer(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1451)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn stop_play_timer(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1452)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_play_timer_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1453)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_play_timer_remaining_time(&self, ) -> Result<::nn::TimeSpanType> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1454)
			.args(())
			;
		let res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_restricted_by_play_timer(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1455)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_play_timer_settings(&self, ) -> Result<::nn::pctl::PlayTimerSettings> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1456)
			.args(())
			;
		let res : Response<::nn::pctl::PlayTimerSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_play_timer_event_to_request_suspension(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1457)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn notify_wrong_pin_code_input_many_times(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1471)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_network_request(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1472)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_unlinked_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1473)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn clear_unlinked_event(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1474)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_all_features(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1601)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn post_enable_all_features(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1602)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_all_features_disabled(&self, ) -> Result<(bool, bool)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1603)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: bool,
			unk1: bool,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn delete_from_free_communication_application_list_for_debug(&self, unk0: ::nn::ncm::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1901)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_free_communication_application_list_for_debug(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1902)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_pairing(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1941)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_play_timer_settings_for_debug(&self, unk0: ::nn::pctl::PlayTimerSettings) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1951)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_play_timer_spent_time_for_test(&self, ) -> Result<::nn::TimeSpanType> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1952)
			.args(())
			;
		let res : Response<::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn request_pairing_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn finish_request_pairing(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::pctl::detail::PairingInfoBase> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2002)
			.args(unk0)
			;
		let res : Response<::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn authorize_pairing_async(&self, unk0: ::nn::pctl::detail::PairingInfoBase) -> Result<(::nn::pctl::detail::AsyncData, KObject)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2003)
			.args(unk0)
			;
		let mut res : Response<::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn finish_authorize_pairing(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::pctl::detail::PairingInfoBase> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2004)
			.args(unk0)
			;
		let res : Response<::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn retrieve_pairing_info_async(&self, ) -> Result<(::nn::pctl::detail::AsyncData, KObject)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2005)
			.args(())
			;
		let mut res : Response<::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn finish_retrieve_pairing_info(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::pctl::detail::PairingInfoBase> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2006)
			.args(unk0)
			;
		let res : Response<::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unlink_pairing_async(&self, unk0: bool) -> Result<(::nn::pctl::detail::AsyncData, KObject)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2007)
			.args(unk0)
			;
		let mut res : Response<::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	// fn finish_unlink_pairing(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_account_mii_image_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn finish_get_account_mii_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_account_mii_image_content_type_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn finish_get_account_mii_image_content_type(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn synchronize_parental_control_settings_async(&self, ) -> Result<(::nn::pctl::detail::AsyncData, KObject)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2013)
			.args(())
			;
		let mut res : Response<::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn finish_synchronize_parental_control_settings(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2014)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn finish_synchronize_parental_control_settings_with_last_updated(&self, unk0: ::nn::pctl::detail::AsyncData) -> Result<::nn::time::PosixTime> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2015)
			.args(unk0)
			;
		let res : Response<::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IParentalControlService {
	unsafe fn from_kobject(obj: KObject) -> IParentalControlService {
		IParentalControlService(Session::from_kobject(obj))
	}
}
