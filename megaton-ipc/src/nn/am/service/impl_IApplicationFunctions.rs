
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IApplicationFunctions(Session);

impl IApplicationFunctions {
	pub fn PopLaunchParameter(&self, unk0: u32) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn CreateApplicationAndPushAndRequestToStart(&self, unk0: ::nn::ncm::ApplicationId, unk1: ::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateApplicationAndPushAndRequestToStartForQuest(&self, unk0: u32, unk1: u32, unk2: ::nn::ncm::ApplicationId, unk3: ::nn::am::service::IStorage) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: ::nn::ncm::ApplicationId,
		}
		let req = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnsureSaveData(&self, unk0: ::nn::account::Uid) -> Result<i64> {
		let req = Request::new(20)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetDesiredLanguage(&self, ) -> Result<::nn::settings::LanguageCode> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<::nn::settings::LanguageCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetTerminateResult(&self, unk0: u32) -> Result<()> {
		let req = Request::new(22)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayVersion(&self, ) -> Result<::nn::oe::DisplayVersion> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<::nn::oe::DisplayVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetLaunchStorageInfoForDebug(&self, ) -> Result<(::nn::ncm::StorageId, ::nn::ncm::StorageId)> {
		let req = Request::new(24)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: ::nn::ncm::StorageId,
			unk1: ::nn::ncm::StorageId,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn ExtendSaveData(&self, unk0: u8, unk1: ::nn::account::Uid, unk2: i64, unk3: i64) -> Result<i64> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: ::nn::account::Uid,
			unk2: i64,
			unk3: i64,
		}
		let req = Request::new(25)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetSaveDataSize(&self, unk0: u8, unk1: ::nn::account::Uid) -> Result<(i64, i64)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: ::nn::account::Uid,
		}
		let req = Request::new(26)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: i64,
			unk3: i64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn BeginBlockingHomeButtonShortAndLongPressed(&self, unk0: i64) -> Result<()> {
		let req = Request::new(30)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EndBlockingHomeButtonShortAndLongPressed(&self, ) -> Result<()> {
		let req = Request::new(31)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn BeginBlockingHomeButton(&self, unk0: i64) -> Result<()> {
		let req = Request::new(32)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EndBlockingHomeButton(&self, ) -> Result<()> {
		let req = Request::new(33)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn NotifyRunning(&self, ) -> Result<bool> {
		let req = Request::new(40)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetPseudoDeviceId(&self, ) -> Result<::nn::util::Uuid> {
		let req = Request::new(50)
			.args(())
			;
		let mut res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetMediaPlaybackStateForApplication(&self, unk0: bool) -> Result<()> {
		let req = Request::new(60)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsGamePlayRecordingSupported(&self, ) -> Result<bool> {
		let req = Request::new(65)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn InitializeGamePlayRecording(&self, unk0: u64, unk1: KObject) -> Result<()> {
		let req = Request::new(66)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetGamePlayRecordingState(&self, unk0: i32) -> Result<()> {
		let req = Request::new(67)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestToShutdown(&self, ) -> Result<()> {
		let req = Request::new(70)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RequestToReboot(&self, ) -> Result<()> {
		let req = Request::new(71)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IApplicationFunctions {
	unsafe fn from_kobject(obj: KObject) -> IApplicationFunctions {
		IApplicationFunctions(Session::from_kobject(obj))
	}
}
