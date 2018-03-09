
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct ICommonStateGetter(Session);

impl AsRef<Session> for ICommonStateGetter {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ICommonStateGetter {
	pub fn GetEventHandle(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ReceiveMessage(&self, ) -> Result<::nn::am::AppletMessage> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<::nn::am::AppletMessage> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetThisAppletKind(&self, ) -> Result<::nn::am::service::AppletKind> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<::nn::am::service::AppletKind> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn AllowToEnterSleep(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DisallowToEnterSleep(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetOperationMode(&self, ) -> Result<u8> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetPerformanceMode(&self, ) -> Result<u32> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetCradleStatus(&self, ) -> Result<u8> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetBootMode(&self, ) -> Result<u8> {
		let req = Request::new(8)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetCurrentFocusState(&self, ) -> Result<u8> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn RequestToAcquireSleepLock(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReleaseSleepLock(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ReleaseSleepLockTransiently(&self, ) -> Result<()> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAcquiredSleepLockEvent(&self, ) -> Result<KObject> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn PushToGeneralChannel(&self, unk0: &::nn::am::service::IStorage) -> Result<()> {
		let req = Request::new(20)
			.args(())
			.copy_handle(unk0.as_ref().as_ref())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetHomeButtonReaderLockAccessor(&self, ) -> Result<::nn::am::service::ILockAccessor> {
		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetReaderLockAccessorEx(&self, unk0: i32) -> Result<::nn::am::service::ILockAccessor> {
		let req = Request::new(31)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn GetCradleFwVersion(&self, ) -> Result<(u32, u32, u32, u32)> {
		let req = Request::new(40)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
			unk3: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn IsVrModeEnabled(&self, ) -> Result<bool> {
		let req = Request::new(50)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetVrModeEnabled(&self, unk0: bool) -> Result<()> {
		let req = Request::new(51)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsInControllerFirmwareUpdateSection(&self, ) -> Result<bool> {
		let req = Request::new(55)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetDefaultDisplayResolution(&self, ) -> Result<(i32, i32)> {
		let req = Request::new(60)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: i32,
			unk1: i32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn GetDefaultDisplayResolutionChangeEvent(&self, ) -> Result<KObject> {
		let req = Request::new(61)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for ICommonStateGetter {
	unsafe fn from_kobject(obj: KObject) -> ICommonStateGetter {
		ICommonStateGetter(Session::from_kobject(obj))
	}
}
