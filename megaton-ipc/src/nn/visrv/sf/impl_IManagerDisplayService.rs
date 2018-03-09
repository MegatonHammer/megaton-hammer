
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IManagerDisplayService(Session);

impl AsRef<Session> for IManagerDisplayService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IManagerDisplayService {
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

	pub fn CreateManagedLayer(&self, unk0: u32, unk1: u64, unk2: ::nn::applet::AppletResourceUserId) -> Result<u64> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(2010)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DestroyManagedLayer(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2011)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateIndirectLayer(&self, ) -> Result<u64> {
		let req = Request::new(2050)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DestroyIndirectLayer(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2051)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateIndirectProducerEndPoint(&self, unk0: u64, unk1: ::nn::applet::AppletResourceUserId) -> Result<u64> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(2052)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DestroyIndirectProducerEndPoint(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2053)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateIndirectConsumerEndPoint(&self, unk0: u64, unk1: ::nn::applet::AppletResourceUserId) -> Result<u64> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(2054)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn DestroyIndirectConsumerEndPoint(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2055)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AcquireLayerTexturePresentingEvent(&self, unk0: u64) -> Result<KObject> {
		let req = Request::new(2300)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn ReleaseLayerTexturePresentingEvent(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2301)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayHotplugEvent(&self, unk0: u64) -> Result<KObject> {
		let req = Request::new(2302)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetDisplayHotplugState(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(2402)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayAlpha(&self, unk0: f32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: u64,
		}
		let req = Request::new(4201)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetDisplayLayerStack(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(4203)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetDisplayPowerState(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(4205)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn AddToLayerStack(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(6000)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RemoveFromLayerStack(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(6001)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetLayerVisibility(&self, unk0: bool, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u64,
		}
		let req = Request::new(6002)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetContentVisibility(&self, unk0: bool) -> Result<()> {
		let req = Request::new(7000)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetConductorLayer(&self, unk0: bool, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u64,
		}
		let req = Request::new(8000)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetIndirectProducerFlipOffset(&self, unk0: u64, unk1: u64, unk2: ::nn::TimeSpan) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
			unk2: ::nn::TimeSpan,
		}
		let req = Request::new(8100)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IManagerDisplayService {
	unsafe fn from_kobject(obj: KObject) -> IManagerDisplayService {
		IManagerDisplayService(Session::from_kobject(obj))
	}
}
