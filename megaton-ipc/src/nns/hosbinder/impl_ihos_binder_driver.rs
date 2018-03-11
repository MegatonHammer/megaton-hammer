
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IHOSBinderDriver(Session);

impl IHOSBinderDriver {
	pub fn new() -> Result<IHOSBinderDriver> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"dispdrv\0").map(|s| unsafe { IHOSBinderDriver::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IHOSBinderDriver {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHOSBinderDriver {
	// fn transact_parcel(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn adjust_refcount(&self, unk0: i32, unk1: i32, unk2: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

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
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_native_handle(&self, unk0: i32, unk1: u32) -> Result<KObject> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn transact_parcel_auto(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IHOSBinderDriver {
	unsafe fn from_kobject(obj: KObject) -> IHOSBinderDriver {
		IHOSBinderDriver(Session::from_kobject(obj))
	}
}
