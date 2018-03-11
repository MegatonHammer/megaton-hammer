
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFriendService(Session);

impl AsRef<Session> for IFriendService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFriendService {
	pub fn get_completion_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn cancel(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_friend_list_ids(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_friend_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn update_friend_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_friend_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send_friend_request_for_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn add_faced_friend_request_for_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_blocked_user_list_ids(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_profile_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn declare_open_online_play_session(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10600)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn declare_close_online_play_session(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10601)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn update_user_presence(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_play_history_registration_key(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_play_history_registration_key_with_network_service_account_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn add_play_history(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_profile_image_url(&self, unk0: ::nn::friends::Url, unk1: i32) -> Result<::nn::friends::Url> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::friends::Url,
			unk1: i32,
		}
		let req = Request::new(11000)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::nn::friends::Url> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_friend_count(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::detail::ipc::SizedFriendFilter, unk2: u64) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::friends::detail::ipc::SizedFriendFilter,
			unk2: u64,
		}
		let req = Request::new(20100)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_newly_friend_count(&self, unk0: ::nn::account::Uid) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20101)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_friend_detailed_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn sync_friend_list(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20103)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_sync_friend_list(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20104)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn load_friend_setting(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_received_friend_request_count(&self, unk0: ::nn::account::Uid) -> Result<(i32, i32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20200)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i32,
			unk2: i32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn get_friend_request_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_friend_candidate_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_nintendo_network_id_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_blocked_user_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn sync_blocked_user_list(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20401)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_profile_extra_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_relationship(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<::nn::friends::Relationship> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(20501)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::nn::friends::Relationship> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_user_presence_view(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_play_history_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_play_history_statistics(&self, unk0: ::nn::account::Uid) -> Result<::nn::friends::PlayHistoryStatistics> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20701)
			.args(unk0)
			;
		let res : Response<::nn::friends::PlayHistoryStatistics> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn load_user_setting(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn sync_user_setting(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20801)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_list_summary_overlay_notification(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20900)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_external_application_catalog(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn drop_friend_newly_flags(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30100)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_friend(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30101)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn drop_friend_newly_flag(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30110)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_friend_favorite_flag(&self, unk0: bool, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::account::Uid,
			unk2: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30120)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_friend_online_notification_flag(&self, unk0: bool, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::account::Uid,
			unk2: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30121)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn send_friend_request(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
			unk2: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30200)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn send_friend_request_with_application_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel_friend_request(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::friends::RequestId,
		}
		let req = Request::new(30202)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn accept_friend_request(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::friends::RequestId,
		}
		let req = Request::new(30203)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reject_friend_request(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::friends::RequestId,
		}
		let req = Request::new(30204)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn read_friend_request(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::friends::RequestId,
		}
		let req = Request::new(30205)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_faced_friend_request_registration_key(&self, unk0: ::nn::account::Uid) -> Result<::nn::friends::FacedFriendRequestRegistrationKey> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30210)
			.args(unk0)
			;
		let res : Response<::nn::friends::FacedFriendRequestRegistrationKey> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn add_faced_friend_request(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel_faced_friend_request(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30212)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_faced_friend_request_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_faced_friend_request_profile_image_from_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send_friend_request_with_external_application_catalog_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn resend_faced_friend_request(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30216)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn send_friend_request_with_nintendo_network_id_info(&self, unk0: ::nn::friends::MiiName, unk1: ::nn::friends::MiiImageUrlParam, unk2: ::nn::friends::MiiName, unk3: ::nn::friends::MiiImageUrlParam, unk4: i32, unk5: ::nn::account::Uid, unk6: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::friends::MiiName,
			unk1: ::nn::friends::MiiImageUrlParam,
			unk2: ::nn::friends::MiiName,
			unk3: ::nn::friends::MiiImageUrlParam,
			unk4: i32,
			unk5: ::nn::account::Uid,
			unk6: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30217)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
				unk4,
				unk5,
				unk6,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn block_user(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
			unk2: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30400)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn block_user_with_application_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unblock_user(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(30402)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_profile_extra_from_friend_code(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn delete_play_history(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30700)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_presence_permission(&self, unk0: i32, unk1: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(30810)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_friend_request_reception(&self, unk0: bool, unk1: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(30811)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_play_log_permission(&self, unk0: i32, unk1: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(30812)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn issue_friend_code(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30820)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_play_log(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30830)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_network_service_account_cache(&self, unk0: ::nn::account::Uid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(49900)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFriendService {
	unsafe fn from_kobject(obj: KObject) -> IFriendService {
		IFriendService(Session::from_kobject(obj))
	}
}
