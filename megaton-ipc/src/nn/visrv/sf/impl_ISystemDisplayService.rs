
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct ISystemDisplayService(Session);

impl ISystemDisplayService {
	pub fn GetZOrderCountMin(&self, unk0: u64) -> Result<i64> {
		let req = Request::new(1200)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetZOrderCountMax(&self, unk0: u64) -> Result<i64> {
		let req = Request::new(1202)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetDisplayLogicalResolution(&self, unk0: u64) -> Result<(i32, i32)> {
		let req = Request::new(1203)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i32,
			unk2: i32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn SetDisplayMagnification(&self, unk0: i32, unk1: i32, unk2: i32, unk3: i32, unk4: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: i32,
			unk3: i32,
			unk4: u64,
		}
		let req = Request::new(1204)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
				unk4,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetLayerPosition(&self, unk0: f32, unk1: f32, unk2: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: f32,
			unk2: u64,
		}
		let req = Request::new(2201)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetLayerSize(&self, unk0: u64, unk1: i64, unk2: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: i64,
			unk2: i64,
		}
		let req = Request::new(2203)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetLayerZ(&self, unk0: u64) -> Result<i64> {
		let req = Request::new(2204)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetLayerZ(&self, unk0: u64, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: i64,
		}
		let req = Request::new(2205)
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
		let req = Request::new(2207)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetLayerAlpha(&self, unk0: f32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: u64,
		}
		let req = Request::new(2209)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn CreateStrayLayer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenIndirectLayer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn CloseIndirectLayer(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2401)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn FlipIndirectLayer(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2402)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ListDisplayModes(&self, unk0: u64, unk2: &mut [::nn::vi::DisplayModeInfo]) -> Result<i64> {
		let req = Request::new(3000)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ListDisplayRgbRanges(&self, unk0: u64, unk2: &mut [u32]) -> Result<i64> {
		let req = Request::new(3001)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ListDisplayContentTypes(&self, unk0: u64, unk2: &mut [u32]) -> Result<i64> {
		let req = Request::new(3002)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetDisplayMode(&self, unk0: u64) -> Result<::nn::vi::DisplayModeInfo> {
		let req = Request::new(3200)
			.args(unk0)
			;
		let mut res : Response<::nn::vi::DisplayModeInfo> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayMode(&self, unk0: u64, unk1: ::nn::vi::DisplayModeInfo) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: ::nn::vi::DisplayModeInfo,
		}
		let req = Request::new(3201)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayUnderscan(&self, unk0: u64) -> Result<i64> {
		let req = Request::new(3202)
			.args(unk0)
			;
		let mut res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayUnderscan(&self, unk0: u64, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: i64,
		}
		let req = Request::new(3203)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayContentType(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(3204)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayContentType(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(3205)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayRgbRange(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(3206)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayRgbRange(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(3207)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayCmuMode(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(3208)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayCmuMode(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(3209)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayContrastRatio(&self, unk0: u64) -> Result<f32> {
		let req = Request::new(3210)
			.args(unk0)
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayContrastRatio(&self, unk0: f32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: u64,
		}
		let req = Request::new(3211)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayGamma(&self, unk0: u64) -> Result<f32> {
		let req = Request::new(3214)
			.args(unk0)
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayGamma(&self, unk0: f32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: u64,
		}
		let req = Request::new(3215)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetDisplayCmuLuma(&self, unk0: u64) -> Result<f32> {
		let req = Request::new(3216)
			.args(unk0)
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn SetDisplayCmuLuma(&self, unk0: f32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: u64,
		}
		let req = Request::new(3217)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISystemDisplayService {
	unsafe fn from_kobject(obj: KObject) -> ISystemDisplayService {
		ISystemDisplayService(Session::from_kobject(obj))
	}
}
