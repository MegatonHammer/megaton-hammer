
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IPcvService(Session);

impl IPcvService {
	pub fn new() -> Result<Arc<IPcvService>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IPcvService>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"pcv\0\0\0\0\0").map(|s| Arc::new(unsafe { IPcvService::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
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
	pub fn set_power_enabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_clock_enabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_clock_rate(&self, unk0: i32, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_clock_rate(&self, unk0: i32) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_state(&self, unk0: i32) -> Result<::nn::pcv::ModuleState> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let res : Response<::nn::pcv::ModuleState> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_possible_clock_rates(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_min_v_clock_rate(&self, unk0: i32, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_reset(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_voltage_enabled(&self, unk0: bool, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_voltage_enabled(&self, unk0: i32) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_voltage_range(&self, unk0: i32) -> Result<(i32, i32, i32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i32,
			unk2: i32,
			unk3: i32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn set_voltage_value(&self, unk0: i32, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_voltage_value(&self, unk0: i32) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_temperature_thresholds(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_temperature(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn initialize(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_initialized(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn finalize(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn power_on(&self, unk0: ::nn::pcv::PowerControlTarget, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn power_off(&self, unk0: ::nn::pcv::PowerControlTarget) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn change_voltage(&self, unk0: ::nn::pcv::PowerControlTarget, unk1: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_power_clock_info_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_oscillator_clock(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_dvfs_table(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_module_state_table(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_power_domain_state_table(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_fuse_info(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IPcvService {
	unsafe fn from_kobject(obj: KObject) -> IPcvService {
		IPcvService(Session::from_kobject(obj))
	}
}
