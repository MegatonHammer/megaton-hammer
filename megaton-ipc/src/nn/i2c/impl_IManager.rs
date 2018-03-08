
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IManager(Session);

impl IManager {
	pub fn get_service() -> Result<IManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"i2c:pcv\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"i2c\0\0\0\0\0").map(|s| unsafe { IManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IManager {
	pub fn OpenSessionForDev(&self, unk0: u16, unk1: u32, unk2: u32, unk3: u32) -> Result<Session> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u16,
			unk1: u32,
			unk2: u32,
			unk3: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenSession(&self, unk0: u32) -> Result<Session> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn HasDevice(&self, unk0: u32) -> Result<u8> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn HasDeviceForDev(&self, unk0: u16, unk1: u32, unk2: u32, unk3: u32) -> Result<u8> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u16,
			unk1: u32,
			unk2: u32,
			unk3: u32,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IManager {
	unsafe fn from_kobject(obj: KObject) -> IManager {
		IManager(Session::from_kobject(obj))
	}
}
