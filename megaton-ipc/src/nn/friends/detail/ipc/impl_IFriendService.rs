
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IFriendService(Session);

impl AsRef<Session> for IFriendService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFriendService {
	pub fn GetCompletionEvent(&self, ) -> Result<KObject> {
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

	// fn GetFriendListIds(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetFriendList(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn UpdateFriendInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetFriendProfileImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SendFriendRequestForApplication(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn AddFacedFriendRequestForApplication(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetBlockedUserListIds(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetProfileList(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn UpdateUserPresence(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetPlayHistoryRegistrationKey(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetPlayHistoryRegistrationKeyWithNetworkServiceAccountId(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn AddPlayHistory(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetProfileImageUrl(&self, unk0: ::nn::friends::Url, unk1: i32) -> Result<::nn::friends::Url> {
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

	pub fn GetFriendCount(&self, unk0: ::nn::account::Uid, unk1: ::nn::friends::detail::ipc::SizedFriendFilter, unk2: u64) -> Result<i32> {
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

	pub fn GetNewlyFriendCount(&self, unk0: ::nn::account::Uid) -> Result<i32> {
		let req = Request::new(20101)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetFriendDetailedInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn LoadFriendSetting(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn GetFriendRequestList(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetFriendCandidateList(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNintendoNetworkIdInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetBlockedUserList(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SyncBlockedUserList(&self, unk0: ::nn::account::Uid) -> Result<()> {
		let req = Request::new(20401)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetProfileExtraList(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetRelationship(&self, unk0: ::nn::account::Uid, unk1: ::nn::account::NetworkServiceAccountId) -> Result<::nn::friends::Relationship> {
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

	// fn GetUserPresenceView(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetPlayHistoryList(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetPlayHistoryStatistics(&self, unk0: ::nn::account::Uid) -> Result<::nn::friends::PlayHistoryStatistics> {
		let req = Request::new(20701)
			.args(unk0)
			;
		let mut res : Response<::nn::friends::PlayHistoryStatistics> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn LoadUserSetting(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn GetExternalApplicationCatalog(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn SendFriendRequestWithApplicationInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	pub fn GetFacedFriendRequestRegistrationKey(&self, unk0: ::nn::account::Uid) -> Result<::nn::friends::FacedFriendRequestRegistrationKey> {
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
	// fn SendFriendRequestWithExternalApplicationCatalogId(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn BlockUserWithApplicationInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
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

	// fn GetProfileExtraFromFriendCode(&self, UNKNOWN) -> Result<UNKNOWN>;
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
