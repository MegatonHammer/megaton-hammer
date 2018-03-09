
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IDeviceOperator(Session);

impl AsRef<Session> for IDeviceOperator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeviceOperator {
	pub fn IsSdCardInserted(&self, ) -> Result<u8> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetSdCardSpeedMode(&self, ) -> Result<u64> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetSdCardCid(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn GetSdCardUserAreaSize(&self, ) -> Result<u64> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn GetSdCardProtectedAreaSize(&self, ) -> Result<u64> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetAndClearSdCardErrorInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetMmcCid(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetMmcSpeedMode(&self, ) -> Result<u64> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EraseMmc(&self, unk0: u32) -> Result<()> {
		let req = Request::new(110)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetMmcPartitionSize(&self, unk0: u32) -> Result<u64> {
		let req = Request::new(111)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn GetMmcPatrolCount(&self, ) -> Result<u32> {
		let req = Request::new(112)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetAndClearMmcErrorInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetMmcExtendedCsd(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn IsGameCardInserted(&self, ) -> Result<u8> {
		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn EraseGameCard(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(201)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetGameCardHandle(&self, ) -> Result<u32> {
		let req = Request::new(202)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetGameCardUpdatePartitionInfo(&self, unk0: u32) -> Result<(u32, ::nn::ApplicationId)> {
		let req = Request::new(203)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			version: u32,
			TID: ::nn::ApplicationId,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().version.clone(),res.get_raw().TID.clone()))
	}

	pub fn FinalizeGameCardDriver(&self, ) -> Result<()> {
		let req = Request::new(204)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetGameCardAttribute(&self, unk0: u32) -> Result<u8> {
		let req = Request::new(205)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetGameCardDeviceCertificate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetGameCardAsicInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetGameCardIdSet(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn WriteToGameCard(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetVerifyWriteEnalbleFlag(&self, flag: u8) -> Result<()> {
		let req = Request::new(210)
			.args(flag)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetGameCardImageHash(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetGameCardDeviceIdForProdCard(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn EraseAndWriteParamDirectly(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetGameCardCid(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn ForceEraseGameCard(&self, ) -> Result<()> {
		let req = Request::new(215)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn GetGameCardErrorInfo(&self, ) -> Result<u128> {
		let req = Request::new(216)
			.args(())
			;
		let mut res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetGameCardErrorReportInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetGameCardDeviceId(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetSpeedEmulationMode(&self, mode: u32) -> Result<()> {
		let req = Request::new(300)
			.args(mode)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetSpeedEmulationMode(&self, ) -> Result<u32> {
		let req = Request::new(301)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDeviceOperator {
	unsafe fn from_kobject(obj: KObject) -> IDeviceOperator {
		IDeviceOperator(Session::from_kobject(obj))
	}
}
