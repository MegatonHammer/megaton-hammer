
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IPcvService(Session);

impl IPcvService {
	pub fn get_service() -> Result<IPcvService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"pcv\0\0\0\0\0").map(|s| unsafe { IPcvService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IPcvService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPcvService {
	pub fn SetPowerEnabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetClockEnabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetClockRate(&self, unk0: i32, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u32,
		}
		let req = Request::new(2)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetClockRate(&self, unk0: i32) -> Result<u32> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetState(&self, unk0: i32) -> Result<::nn::pcv::ModuleState> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<::nn::pcv::ModuleState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetPossibleClockRates(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetMinVClockRate(&self, unk0: i32, unk1: u32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u32,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetReset(&self, unk0: bool, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(7)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetVoltageEnabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i32,
		}
		let req = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetVoltageEnabled(&self, unk0: i32) -> Result<bool> {
		let req = Request::new(9)
			.args(unk0)
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetVoltageRange(&self, unk0: i32) -> Result<(i32, i32, i32)> {
		let req = Request::new(10)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i32,
			unk2: i32,
			unk3: i32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn SetVoltageValue(&self, unk0: i32, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
		}
		let req = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetVoltageValue(&self, unk0: i32) -> Result<i32> {
		let req = Request::new(12)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetTemperatureThresholds(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetTemperature(&self, unk0: i32) -> Result<()> {
		let req = Request::new(14)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Initialize(&self, ) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsInitialized(&self, ) -> Result<bool> {
		let req = Request::new(16)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Finalize(&self, ) -> Result<()> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PowerOn(&self, unk0: ::nn::pcv::PowerControlTarget, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::pcv::PowerControlTarget,
			unk1: i32,
		}
		let req = Request::new(18)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PowerOff(&self, unk0: ::nn::pcv::PowerControlTarget) -> Result<()> {
		let req = Request::new(19)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ChangeVoltage(&self, unk0: ::nn::pcv::PowerControlTarget, unk1: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::pcv::PowerControlTarget,
			unk1: i32,
		}
		let req = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetPowerClockInfoEvent(&self, ) -> Result<KObject> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn GetOscillatorClock(&self, ) -> Result<u32> {
		let req = Request::new(22)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn GetDvfsTable(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetModuleStateTable(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetPowerDomainStateTable(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetFuseInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IPcvService {
	unsafe fn from_kobject(obj: KObject) -> IPcvService {
		IPcvService(Session::from_kobject(obj))
	}
}
