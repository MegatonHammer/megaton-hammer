
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IFriendService<T>(T);

impl IFriendService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IFriendService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFriendService(domain)),
			Err((sess, err)) => Err((IFriendService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFriendService<Session>> {
		Ok(IFriendService(self.0.duplicate()?))
	}
}

impl<T> Deref for IFriendService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFriendService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFriendService<T> {
	pub fn get_completion_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn cancel(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_friend_list_ids(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::friends::detail::ipc::SizedFriendFilter, unk3: u64, unk6: &mut [::ipcdefs::nn::account::NetworkServiceAccountId]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::friends::detail::ipc::SizedFriendFilter,
			unk3: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10100)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_mut_slice(unk6, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_friend_list(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::friends::detail::ipc::SizedFriendFilter, unk3: u64, unk6: &mut [::ipcdefs::nn::friends::detail::FriendImpl]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::friends::detail::ipc::SizedFriendFilter,
			unk3: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10101)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_mut_slice(unk6, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn update_friend_info(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: u64, unk3: &[::ipcdefs::nn::account::NetworkServiceAccountId], unk4: &mut [::ipcdefs::nn::friends::detail::FriendImpl]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: u64,
		}
		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(10102)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_slice(unk3, 9))
			.descriptor(IPCBuffer::from_mut_slice(unk4, 6))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_friend_profile_image(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn send_friend_request_for_application(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId, unk2: u64, unk4: &::ipcdefs::nn::friends::InAppScreenName, unk5: &::ipcdefs::nn::friends::InAppScreenName) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
			unk2: u64,
		}
		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(10200)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_ref(unk4, 0x19))
			.descriptor(IPCBuffer::from_ref(unk5, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn add_faced_friend_request_for_application(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_blocked_user_list_ids(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk3: &mut [::ipcdefs::nn::account::NetworkServiceAccountId]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10400)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk3, 0xa))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_profile_list(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: &[::ipcdefs::nn::account::NetworkServiceAccountId], unk2: &mut [::ipcdefs::nn::friends::detail::ProfileImpl]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(10500)
			.args(unk0)
			.descriptor(IPCBuffer::from_slice(unk1, 9))
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn declare_open_online_play_session(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10600)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn declare_close_online_play_session(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10601)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn update_user_presence(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: u64, unk3: &::ipcdefs::nn::friends::detail::UserPresenceImpl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: u64,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10610)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_ref(unk3, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_play_history_registration_key(&self, unk0: bool, unk1: ::ipcdefs::nn::account::Uid, unk2: &mut ::ipcdefs::nn::friends::PlayHistoryRegistrationKey) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10700)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_play_history_registration_key_with_network_service_account_id(&self, unk0: bool, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId, unk2: &mut ::ipcdefs::nn::friends::PlayHistoryRegistrationKey) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(10701)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn add_play_history(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: u64, unk3: &::ipcdefs::nn::friends::PlayHistoryRegistrationKey, unk4: &::ipcdefs::nn::friends::InAppScreenName, unk5: &::ipcdefs::nn::friends::InAppScreenName) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: u64,
		}
		let req : Request<_, [_; 3], [_; 0], [_; 0]> = Request::new(10702)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_ref(unk3, 0x19))
			.descriptor(IPCBuffer::from_ref(unk4, 0x19))
			.descriptor(IPCBuffer::from_ref(unk5, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_profile_image_url(&self, unk0: ::ipcdefs::nn::friends::Url, unk1: i32) -> Result<::ipcdefs::nn::friends::Url> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::friends::Url,
			unk1: i32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11000)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::ipcdefs::nn::friends::Url> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_friend_count(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::friends::detail::ipc::SizedFriendFilter, unk2: u64) -> Result<i32> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::friends::detail::ipc::SizedFriendFilter,
			unk2: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20100)
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

	pub fn get_newly_friend_count(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20101)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_friend_detailed_info(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId, unk2: &mut ::ipcdefs::nn::friends::detail::FriendDetailedInfoImpl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20102)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn sync_friend_list(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20103)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_sync_friend_list(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20104)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn load_friend_setting(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId, unk2: &mut ::ipcdefs::nn::friends::detail::FriendSettingImpl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20110)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_received_friend_request_count(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<(i32, i32)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20200)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i32,
			unk2: i32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn get_friend_request_list(&self, unk0: i32, unk1: i32, unk2: ::ipcdefs::nn::account::Uid, unk4: &mut [::ipcdefs::nn::friends::detail::FriendRequestImpl]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20201)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk4, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_friend_candidate_list(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk3: &mut [::ipcdefs::nn::friends::detail::FriendCandidateImpl]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20300)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk3, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_nintendo_network_id_info(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk3: &mut ::ipcdefs::nn::friends::NintendoNetworkIdUserInfo, unk4: &mut [::ipcdefs::nn::friends::detail::NintendoNetworkIdFriendImpl]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(20301)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_ref(unk3, 0x1a))
			.descriptor(IPCBuffer::from_mut_slice(unk4, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_sns_account_linkage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_sns_account_profile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_sns_account_friend_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_blocked_user_list(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk3: &mut [::ipcdefs::nn::friends::detail::BlockedUserImpl]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20400)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk3, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn sync_blocked_user_list(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20401)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_profile_extra_list(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: &[::ipcdefs::nn::account::NetworkServiceAccountId], unk2: &mut [::ipcdefs::nn::friends::detail::ProfileExtraImpl]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(20500)
			.args(unk0)
			.descriptor(IPCBuffer::from_slice(unk1, 9))
			.descriptor(IPCBuffer::from_mut_slice(unk2, 6))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_relationship(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<::ipcdefs::nn::friends::Relationship> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20501)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::ipcdefs::nn::friends::Relationship> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_user_presence_view(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: &mut ::ipcdefs::nn::friends::detail::UserPresenceViewImpl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20600)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_play_history_list(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk3: &mut [::ipcdefs::nn::friends::detail::PlayHistoryImpl]) -> Result<i32> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20700)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_slice(unk3, 6))
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_play_history_statistics(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::friends::PlayHistoryStatistics> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20701)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::friends::PlayHistoryStatistics> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn load_user_setting(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: &mut ::ipcdefs::nn::friends::detail::UserSettingImpl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(20800)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn sync_user_setting(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20801)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_list_summary_overlay_notification(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20900)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_external_application_catalog(&self, unk0: ::ipcdefs::nn::settings::LanguageCode, unk1: ::ipcdefs::nn::friends::ExternalApplicationCatalogId, unk2: &mut ::ipcdefs::nn::friends::ExternalApplicationCatalog) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::settings::LanguageCode,
			unk1: ::ipcdefs::nn::friends::ExternalApplicationCatalogId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(21000)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn drop_friend_newly_flags(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30100)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_friend(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30101)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn drop_friend_newly_flag(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30110)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_friend_favorite_flag(&self, unk0: bool, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30120)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_friend_online_notification_flag(&self, unk0: bool, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30121)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn send_friend_request(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30200)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn send_friend_request_with_application_info(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::account::NetworkServiceAccountId, unk3: ::ipcdefs::nn::friends::ApplicationInfo, unk4: &::ipcdefs::nn::friends::InAppScreenName, unk5: &::ipcdefs::nn::friends::InAppScreenName) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::account::NetworkServiceAccountId,
			unk3: ::ipcdefs::nn::friends::ApplicationInfo,
		}
		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(30201)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.descriptor(IPCBuffer::from_ref(unk4, 0x19))
			.descriptor(IPCBuffer::from_ref(unk5, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_friend_request(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::friends::RequestId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::friends::RequestId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30202)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn accept_friend_request(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::friends::RequestId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::friends::RequestId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30203)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reject_friend_request(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::friends::RequestId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::friends::RequestId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30204)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn read_friend_request(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::friends::RequestId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::friends::RequestId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30205)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_faced_friend_request_registration_key(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<::ipcdefs::nn::friends::FacedFriendRequestRegistrationKey> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30210)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::friends::FacedFriendRequestRegistrationKey> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn add_faced_friend_request(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn cancel_faced_friend_request(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30212)
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
	pub fn send_friend_request_with_external_application_catalog_id(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::account::NetworkServiceAccountId, unk3: ::ipcdefs::nn::friends::ExternalApplicationCatalogId, unk4: &::ipcdefs::nn::friends::InAppScreenName, unk5: &::ipcdefs::nn::friends::InAppScreenName) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::account::NetworkServiceAccountId,
			unk3: ::ipcdefs::nn::friends::ExternalApplicationCatalogId,
		}
		let req : Request<_, [_; 2], [_; 0], [_; 0]> = Request::new(30215)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.descriptor(IPCBuffer::from_ref(unk4, 0x19))
			.descriptor(IPCBuffer::from_ref(unk5, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn resend_faced_friend_request(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30216)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn send_friend_request_with_nintendo_network_id_info(&self, unk0: ::ipcdefs::nn::friends::MiiName, unk1: ::ipcdefs::nn::friends::MiiImageUrlParam, unk2: ::ipcdefs::nn::friends::MiiName, unk3: ::ipcdefs::nn::friends::MiiImageUrlParam, unk4: i32, unk5: ::ipcdefs::nn::account::Uid, unk6: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::friends::MiiName,
			unk1: ::ipcdefs::nn::friends::MiiImageUrlParam,
			unk2: ::ipcdefs::nn::friends::MiiName,
			unk3: ::ipcdefs::nn::friends::MiiImageUrlParam,
			unk4: i32,
			unk5: ::ipcdefs::nn::account::Uid,
			unk6: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30217)
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

	// fn get_sns_account_link_page_url(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unlink_sns_account(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn block_user(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30400)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn block_user_with_application_info(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid, unk2: ::ipcdefs::nn::account::NetworkServiceAccountId, unk3: ::ipcdefs::nn::friends::ApplicationInfo, unk4: &::ipcdefs::nn::friends::InAppScreenName) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
			unk2: ::ipcdefs::nn::account::NetworkServiceAccountId,
			unk3: ::ipcdefs::nn::friends::ApplicationInfo,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(30401)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.descriptor(IPCBuffer::from_ref(unk4, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unblock_user(&self, unk0: ::ipcdefs::nn::account::Uid, unk1: ::ipcdefs::nn::account::NetworkServiceAccountId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::account::Uid,
			unk1: ::ipcdefs::nn::account::NetworkServiceAccountId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30402)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_profile_extra_from_friend_code(&self, unk0: ::ipcdefs::nn::friends::FriendCode, unk1: ::ipcdefs::nn::account::Uid, unk2: &mut ::ipcdefs::nn::friends::detail::ProfileExtraImpl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::friends::FriendCode,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(30500)
			.args(InRaw {
				unk0,
				unk1,
			})
			.descriptor(IPCBuffer::from_mut_ref(unk2, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_play_history(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30700)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_presence_permission(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30810)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_friend_request_reception(&self, unk0: bool, unk1: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30811)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_play_log_permission(&self, unk0: i32, unk1: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::ipcdefs::nn::account::Uid,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30812)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn issue_friend_code(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30820)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_play_log(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30830)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_network_service_account_cache(&self, unk0: ::ipcdefs::nn::account::Uid) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(49900)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IFriendService<T> {
	fn from(obj: T) -> IFriendService<T> {
		IFriendService(obj)
	}
}
