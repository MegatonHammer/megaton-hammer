
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IApplicationDisplayService(Session);

impl IApplicationDisplayService {
	pub fn GetRelayService(&self, ) -> Result<::nns::hosbinder::IHOSBinderDriver> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn GetSystemDisplayService(&self, ) -> Result<::nn::visrv::sf::ISystemDisplayService> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn GetManagerDisplayService(&self, ) -> Result<::nn::visrv::sf::IManagerDisplayService> {
		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	#[cfg(feature = "switch-2.0.0")]
	pub fn GetIndirectDisplayTransactionService(&self, ) -> Result<::nns::hosbinder::IHOSBinderDriver> {
		let req = Request::new(103)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	pub fn ListDisplays(&self, unk1: &mut [::nn::vi::DisplayInfo]) -> Result<i64> {
		let req = Request::new(1000)
			.args(())
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn OpenDisplay(&self, unk0: ::nn::vi::DisplayName) -> Result<u64> {
		let req = Request::new(1010)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn OpenDefaultDisplay(&self, ) -> Result<u64> {
		let req = Request::new(1011)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn CloseDisplay(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1020)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetDisplayEnabled(&self, unk0: bool, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u64,
		}
		let req = Request::new(1101)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetDisplayResolution(&self, unk0: u64) -> Result<(i64, i64)> {
		let req = Request::new(1102)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i64,
			unk2: i64,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}
	// fn OpenLayer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn CloseLayer(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2021)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn CreateStrayLayer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn DestroyStrayLayer(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2031)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn SetLayerScalingMode(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(2101)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetIndirectLayerImageMap(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetIndirectLayerImageCropMap(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetIndirectLayerImageRequiredMemoryInfo(&self, unk0: i64, unk1: i64) -> Result<(i64, i64)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i64,
			unk1: i64,
		}
		let req = Request::new(2460)
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
	// fn GetDisplayVsyncEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetDisplayVsyncEventForDebug(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IApplicationDisplayService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationDisplayService {
		IApplicationDisplayService(Session::from_kobject(obj))
	}
}
