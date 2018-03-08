
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IHOSBinderDriver(Session);

impl IHOSBinderDriver {
	pub fn get_service() -> Result<IHOSBinderDriver> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"dispdrv\0").map(|s| unsafe { IHOSBinderDriver::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IHOSBinderDriver {
	// fn TransactParcel(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn AdjustRefcount(&self, unk0: i32, unk1: i32, unk2: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: i32,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetNativeHandle(&self, unk0: i32, unk1: u32) -> Result<KObject> {
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
		Ok(res.pop_handle())
	}

	// fn TransactParcelAuto(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IHOSBinderDriver {
	unsafe fn from_kobject(obj: KObject) -> IHOSBinderDriver {
		IHOSBinderDriver(Session::from_kobject(obj))
	}
}
