
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IFriendService(Session);

impl IFriendService {
	pub fn GetCompletionEvent(&self, ) -> Result<(KObject)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Cancel(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetFriendListIds(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::friends::detail::ipc::SizedFriendFilter, unk3: u64, unk6: &mut [::nn::account::NetworkServiceAccountId]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
			unk2: ::nn::friends::detail::ipc::SizedFriendFilter,
			unk3: u64,
		}
		let req = Request::new(10100)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetFriendList(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::friends::detail::ipc::SizedFriendFilter, unk3: u64, unk6: &mut [::nn::friends::detail::FriendImpl]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
			unk2: ::nn::friends::detail::ipc::SizedFriendFilter,
			unk3: u64,
		}
		let req = Request::new(10101)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn UpdateFriendInfo(&self, unk0: ::nn::account::Uid, unk1: u64, unk3: &[::nn::account::NetworkServiceAccountId], unk4: &mut [::nn::friends::detail::FriendImpl]) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: u64,
		}
		let req = Request::new(10102)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetFriendProfileImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SendFriendRequestForApplication(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId, unk2: u64, unk4: &::nn::friends::InAppScreenName, unk5: &::nn::friends::InAppScreenName) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
			unk2: u64,
		}
		let req = Request::new(10200)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn AddFacedFriendRequestForApplication(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetBlockedUserListIds(&self, unk0: i32, unk1: ::nn::account::Uid, unk3: &mut [::nn::account::NetworkServiceAccountId]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(10400)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetProfileList(&self, unk0: ::nn::account::Uid, unk1: &[::nn::account::NetworkServiceAccountId], unk2: &mut [::nn::friends::detail::ProfileImpl]) -> Result<()> {
		let req = Request::new(10500)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeclareOpenOnlinePlaySession(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(10600)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeclareCloseOnlinePlaySession(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(10601)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UpdateUserPresence(&self, unk0: ::nn::account::Uid, unk1: u64, unk3: &::nn::friends::detail::UserPresenceImpl) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: u64,
		}
		let req = Request::new(10610)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPlayHistoryRegistrationKey(&self, unk0: bool, unk1: ::nn::account::Uid, unk2: &mut Option<::nn::friends::PlayHistoryRegistrationKey>) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(10700)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPlayHistoryRegistrationKeyWithNetworkServiceAccountId(&self, unk0: bool, unk1: ::nn::account::NetworkServiceAccountId, unk2: &mut Option<::nn::friends::PlayHistoryRegistrationKey>) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(10701)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AddPlayHistory(&self, unk0: ::nn::account::Uid, unk1: u64, unk3: &::nn::friends::PlayHistoryRegistrationKey, unk4: &::nn::friends::InAppScreenName, unk5: &::nn::friends::InAppScreenName) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: u64,
		}
		let req = Request::new(10702)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetProfileImageUrl(&self, unk0: ::nn::friends::Url, unk1: i32) -> Result<(::nn::friends::Url)> {
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
		let mut res : Response<::nn::friends::Url> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetFriendCount(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::detail::ipc::SizedFriendFilter, unk2: u64) -> Result<(i32)> {
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
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetNewlyFriendCount(&self, unk0: ::nn::account::Uid) -> Result<(i32)> {
		let req = Request::new(20101)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetFriendDetailedInfo(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId, unk2: &mut Option<::nn::friends::detail::FriendDetailedInfoImpl>) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(20102)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SyncFriendList(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(20103)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestSyncFriendList(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(20104)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn LoadFriendSetting(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId, unk2: &mut Option<::nn::friends::detail::FriendSettingImpl>) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::account::Uid,
			unk1: ::nn::account::NetworkServiceAccountId,
		}
		let req = Request::new(20110)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetReceivedFriendRequestCount(&self, unk0: ::nn::account::Uid) -> Result<(i32, i32)> {
		let req = Request::new(20200)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i32,
			unk2: i32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn GetFriendRequestList(&self, unk0: i32, unk1: i32, unk2: ::nn::account::Uid, unk4: &mut [::nn::friends::detail::FriendRequestImpl]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: ::nn::account::Uid,
		}
		let req = Request::new(20201)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetFriendCandidateList(&self, unk0: i32, unk1: ::nn::account::Uid, unk3: &mut [::nn::friends::detail::FriendCandidateImpl]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(20300)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetNintendoNetworkIdInfo(&self, unk0: i32, unk1: ::nn::account::Uid, unk3: &mut Option<::nn::friends::NintendoNetworkIdUserInfo>, unk4: &mut [::nn::friends::detail::NintendoNetworkIdFriendImpl]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(20301)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetBlockedUserList(&self, unk0: i32, unk1: ::nn::account::Uid, unk3: &mut [::nn::friends::detail::BlockedUserImpl]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(20400)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SyncBlockedUserList(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(20401)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetProfileExtraList(&self, unk0: ::nn::account::Uid, unk1: &[::nn::account::NetworkServiceAccountId], unk2: &mut [::nn::friends::detail::ProfileExtraImpl]) -> Result<()> {
		let req = Request::new(20500)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetRelationship(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<(::nn::friends::Relationship)> {
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
		let mut res : Response<::nn::friends::Relationship> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetUserPresenceView(&self, unk0: ::nn::account::Uid, unk1: &mut Option<::nn::friends::detail::UserPresenceViewImpl>) -> Result<()> {
		let req = Request::new(20600)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPlayHistoryList(&self, unk0: i32, unk1: ::nn::account::Uid, unk3: &mut [::nn::friends::detail::PlayHistoryImpl]) -> Result<(i32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(20700)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetPlayHistoryStatistics(&self, unk0: ::nn::account::Uid) -> Result<(::nn::friends::PlayHistoryStatistics)> {
		let req = Request::new(20701)
			.args(unk0)
			;
		let mut res : Response<::nn::friends::PlayHistoryStatistics> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn LoadUserSetting(&self, unk0: ::nn::account::Uid, unk1: &mut Option<::nn::friends::detail::UserSettingImpl>) -> Result<()> {
		let req = Request::new(20800)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SyncUserSetting(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(20801)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestListSummaryOverlayNotification(&self, ) -> Result<()> {
		let req = Request::new(20900)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetExternalApplicationCatalog(&self, unk0: ::nn::settings::LanguageCode, unk1: ::nn::friends::ExternalApplicationCatalogId, unk2: &mut Option<::nn::friends::ExternalApplicationCatalog>) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::settings::LanguageCode,
			unk1: ::nn::friends::ExternalApplicationCatalogId,
		}
		let req = Request::new(21000)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DropFriendNewlyFlags(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(30100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteFriend(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DropFriendNewlyFlag(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ChangeFriendFavoriteFlag(&self, unk0: bool, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ChangeFriendOnlineNotificationFlag(&self, unk0: bool, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SendFriendRequest(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SendFriendRequestWithApplicationInfo(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId, unk3: ::nn::friends::ApplicationInfo, unk4: &::nn::friends::InAppScreenName, unk5: &::nn::friends::InAppScreenName) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
			unk2: ::nn::account::NetworkServiceAccountId,
			unk3: ::nn::friends::ApplicationInfo,
		}
		let req = Request::new(30201)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CancelFriendRequest(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcceptFriendRequest(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RejectFriendRequest(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReadFriendRequest(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::RequestId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetFacedFriendRequestRegistrationKey(&self, unk0: ::nn::account::Uid) -> Result<(::nn::friends::FacedFriendRequestRegistrationKey)> {
		let req = Request::new(30210)
			.args(unk0)
			;
		let mut res : Response<::nn::friends::FacedFriendRequestRegistrationKey> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn AddFacedFriendRequest(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn CancelFacedFriendRequest(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetFacedFriendRequestProfileImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetFacedFriendRequestProfileImageFromPath(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SendFriendRequestWithExternalApplicationCatalogId(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId, unk3: ::nn::friends::ExternalApplicationCatalogId, unk4: &::nn::friends::InAppScreenName, unk5: &::nn::friends::InAppScreenName) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
			unk2: ::nn::account::NetworkServiceAccountId,
			unk3: ::nn::friends::ExternalApplicationCatalogId,
		}
		let req = Request::new(30215)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ResendFacedFriendRequest(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SendFriendRequestWithNintendoNetworkIdInfo(&self, unk0: ::nn::friends::MiiName, unk1: ::nn::friends::MiiImageUrlParam, unk2: ::nn::friends::MiiName, unk3: ::nn::friends::MiiImageUrlParam, unk4: i32, unk5: ::nn::account::Uid, unk6: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn BlockUser(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn BlockUserWithApplicationInfo(&self, unk0: i32, unk1: ::nn::account::Uid, unk2: ::nn::account::NetworkServiceAccountId, unk3: ::nn::friends::ApplicationInfo, unk4: &::nn::friends::InAppScreenName) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::account::Uid,
			unk2: ::nn::account::NetworkServiceAccountId,
			unk3: ::nn::friends::ApplicationInfo,
		}
		let req = Request::new(30401)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnblockUser(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetProfileExtraFromFriendCode(&self, unk0: ::nn::friends::FriendCode, unk1: ::nn::account::Uid, unk2: &mut Option<::nn::friends::detail::ProfileExtraImpl>) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::friends::FriendCode,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(30500)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeletePlayHistory(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(30700)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ChangePresencePermission(&self, unk0: i32, unk1: ::nn::account::Uid) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ChangeFriendRequestReception(&self, unk0: bool, unk1: ::nn::account::Uid) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ChangePlayLogPermission(&self, unk0: i32, unk1: ::nn::account::Uid) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IssueFriendCode(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(30820)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ClearPlayLog(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(30830)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteNetworkServiceAccountCache(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(49900)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IFriendService {
	unsafe fn from_kobject(obj: KObject) -> IFriendService {
		IFriendService(Session::from_kobject(obj))
	}
}
