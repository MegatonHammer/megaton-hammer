
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IParentalControlService<T>(T);

impl IParentalControlService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IParentalControlService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IParentalControlService(domain)),
			Err((sess, err)) => Err((IParentalControlService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IParentalControlService<Session>> {
		Ok(IParentalControlService(self.0.duplicate()?))
	}
}

impl<T> Deref for IParentalControlService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IParentalControlService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IParentalControlService<T> {
	#[cfg(feature = "switch-4.0.0")]
	pub fn initialize(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn check_free_communication_permission(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1001)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn confirm_launch_application_permission(&self, unk0: bool, unk1: ::ipcdefs::nn::ncm::ApplicationId, unk2: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::ncm::ApplicationId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1002)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_slice(unk2, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn confirm_resume_application_permission(&self, unk0: bool, unk1: ::ipcdefs::nn::ncm::ApplicationId, unk2: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::ncm::ApplicationId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1003)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_slice(unk2, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn confirm_sns_post_permission(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1004)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn confirm_system_settings_permission(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1005)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_restriction_temporary_unlocked(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1006)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn revert_restriction_temporary_unlocked(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1007)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enter_restricted_system_settings(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1008)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn leave_restricted_system_settings(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1009)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_restricted_system_settings_entered(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1010)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn revert_restricted_system_settings_entered(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1011)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_restricted_features(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1012)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn confirm_stereo_vision_permission(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1013)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn confirm_playable_application_video_old(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1014)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn confirm_playable_application_video(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1015)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_restriction_enabled(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1031)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_safety_level(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1032)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_safety_level(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1033)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_safety_level_settings(&self, unk0: i32) -> Result<::ipcdefs::nn::pctl::SafetyLevelSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1034)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::pctl::SafetyLevelSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_settings(&self, ) -> Result<::ipcdefs::nn::pctl::SafetyLevelSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1035)
			.args(())
			;
		let res : Response<::ipcdefs::nn::pctl::SafetyLevelSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_custom_safety_level_settings(&self, unk0: ::ipcdefs::nn::pctl::SafetyLevelSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1036)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_default_rating_organization(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1037)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_default_rating_organization(&self, unk0: i32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1038)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_free_communication_application_list_count(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1039)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn add_to_free_communication_application_list(&self, unk0: ::ipcdefs::nn::ncm::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1042)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_settings(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1043)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_free_communication_application_list(&self, unk0: i32, unk2: &mut [::ipcdefs::nn::pctl::FreeCommunicationApplicationInfo]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1044)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn update_free_communication_application_list(&self, unk0: &[::ipcdefs::nn::pctl::FreeCommunicationApplicationInfo]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1045)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_features_for_reset(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1046)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn notify_application_download_started(&self, unk0: ::ipcdefs::nn::ncm::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1047)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn confirm_stereo_vision_restriction_configurable(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1061)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_stereo_vision_restriction(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1062)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_stereo_vision_restriction(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1063)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn reset_confirmed_stereo_vision_permission(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1064)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn is_stereo_vision_permitted(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1065)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unlock_restriction_temporarily(&self, unk0: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1201)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unlock_system_settings_restriction(&self, unk0: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1202)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_pin_code(&self, unk0: &[i8]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1203)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn generate_inquiry_code(&self, ) -> Result<::ipcdefs::nn::pctl::InquiryCode> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1204)
			.args(())
			;
		let res : Response<::ipcdefs::nn::pctl::InquiryCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn check_master_key(&self, unk0: ::ipcdefs::nn::pctl::InquiryCode, unk1: &[i8]) -> Result<bool> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1205)
			.args(unk0)
			.descriptor(IPCBuffer::from_slice(unk1, 9))
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_pin_code_length(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1206)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_pin_code_changed_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1207)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_pin_code(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1208)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_pairing_active(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1403)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_settings_last_updated(&self, ) -> Result<::ipcdefs::nn::time::PosixTime> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1406)
			.args(())
			;
		let res : Response<::ipcdefs::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_pairing_account_info(&self, unk0: ::ipcdefs::nn::pctl::detail::PairingInfoBase) -> Result<::ipcdefs::nn::pctl::detail::PairingAccountInfoBase> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1411)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::pctl::detail::PairingAccountInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_account_nickname(&self, unk0: ::ipcdefs::nn::pctl::detail::PairingAccountInfoBase, unk2: &mut [i8]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1421)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_account_state(&self, unk0: ::ipcdefs::nn::pctl::detail::PairingAccountInfoBase) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1424)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_synchronization_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1432)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn start_play_timer(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1451)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn stop_play_timer(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1452)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_play_timer_enabled(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1453)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_play_timer_remaining_time(&self, ) -> Result<::ipcdefs::nn::TimeSpanType> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1454)
			.args(())
			;
		let res : Response<::ipcdefs::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_restricted_by_play_timer(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1455)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_play_timer_settings(&self, ) -> Result<::ipcdefs::nn::pctl::PlayTimerSettings> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1456)
			.args(())
			;
		let res : Response<::ipcdefs::nn::pctl::PlayTimerSettings> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_play_timer_event_to_request_suspension(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1457)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn is_play_timer_alarm_disabled(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1458)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn notify_wrong_pin_code_input_many_times(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1471)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_network_request(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1472)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_unlinked_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1473)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn clear_unlinked_event(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1474)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_all_features(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1601)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn post_enable_all_features(&self, ) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1602)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_all_features_disabled(&self, ) -> Result<(bool, bool)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1603)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: bool,
			unk1: bool,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn delete_from_free_communication_application_list_for_debug(&self, unk0: ::ipcdefs::nn::ncm::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1901)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_free_communication_application_list_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1902)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_exempt_application_list_count_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1903)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_exempt_application_list_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1904)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn update_exempt_application_list_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1905)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn add_to_exempt_application_list_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1906)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn delete_from_exempt_application_list_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1907)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn clear_exempt_application_list_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1908)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_pairing(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1941)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_play_timer_settings_for_debug(&self, unk0: ::ipcdefs::nn::pctl::PlayTimerSettings) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1951)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_play_timer_spent_time_for_test(&self, ) -> Result<::ipcdefs::nn::TimeSpanType> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1952)
			.args(())
			;
		let res : Response<::ipcdefs::nn::TimeSpanType> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_play_timer_alarm_disabled_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1953)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_pairing_async(&self, unk0: &[i8]) -> Result<(::ipcdefs::nn::pctl::detail::AsyncData, KObject)> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2001)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 9))
			;
		let mut res : Response<::ipcdefs::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn finish_request_pairing(&self, unk0: ::ipcdefs::nn::pctl::detail::AsyncData) -> Result<::ipcdefs::nn::pctl::detail::PairingInfoBase> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2002)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn authorize_pairing_async(&self, unk0: ::ipcdefs::nn::pctl::detail::PairingInfoBase) -> Result<(::ipcdefs::nn::pctl::detail::AsyncData, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2003)
			.args(unk0)
			;
		let mut res : Response<::ipcdefs::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn finish_authorize_pairing(&self, unk0: ::ipcdefs::nn::pctl::detail::AsyncData) -> Result<::ipcdefs::nn::pctl::detail::PairingInfoBase> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2004)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn retrieve_pairing_info_async(&self, ) -> Result<(::ipcdefs::nn::pctl::detail::AsyncData, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2005)
			.args(())
			;
		let mut res : Response<::ipcdefs::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn finish_retrieve_pairing_info(&self, unk0: ::ipcdefs::nn::pctl::detail::AsyncData) -> Result<::ipcdefs::nn::pctl::detail::PairingInfoBase> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2006)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::pctl::detail::PairingInfoBase> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unlink_pairing_async(&self, unk0: bool) -> Result<(::ipcdefs::nn::pctl::detail::AsyncData, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2007)
			.args(unk0)
			;
		let mut res : Response<::ipcdefs::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	// fn finish_unlink_pairing(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_account_mii_image_async(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn finish_get_account_mii_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_account_mii_image_content_type_async(&self, unk0: ::ipcdefs::nn::pctl::detail::PairingAccountInfoBase, unk4: &mut [i8]) -> Result<(::ipcdefs::nn::pctl::detail::AsyncData, u32, KObject)> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2011)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk4, 0xa))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: ::ipcdefs::nn::pctl::detail::AsyncData,
			unk2: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.pop_handle()))
	}

	pub fn finish_get_account_mii_image_content_type(&self, unk0: ::ipcdefs::nn::pctl::detail::AsyncData, unk2: &mut [i8]) -> Result<u32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(2012)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_slice(unk2, 0xa))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn synchronize_parental_control_settings_async(&self, ) -> Result<(::ipcdefs::nn::pctl::detail::AsyncData, KObject)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2013)
			.args(())
			;
		let mut res : Response<::ipcdefs::nn::pctl::detail::AsyncData> = self.0.send(req)?;
		Ok((*res.get_raw(),res.pop_handle()))
	}

	pub fn finish_synchronize_parental_control_settings(&self, unk0: ::ipcdefs::nn::pctl::detail::AsyncData) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2014)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn finish_synchronize_parental_control_settings_with_last_updated(&self, unk0: ::ipcdefs::nn::pctl::detail::AsyncData) -> Result<::ipcdefs::nn::time::PosixTime> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2015)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::time::PosixTime> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn request_update_exemption_list_async(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2016)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IParentalControlService<T> {
	fn from(obj: T) -> IParentalControlService<T> {
		IParentalControlService(obj)
	}
}
