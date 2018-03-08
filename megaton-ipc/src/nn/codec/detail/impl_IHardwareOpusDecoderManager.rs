
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IHardwareOpusDecoderManager(Session);

impl IHardwareOpusDecoderManager {
	pub fn get_service() -> Result<IHardwareOpusDecoderManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"hwopus\0\0").map(|s| unsafe { IHardwareOpusDecoderManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IHardwareOpusDecoderManager {
	pub fn Unknown0(&self, unk0: u64, unk1: u32, unk2: KObject) -> Result<Session> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown1(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown2(&self, unk0: u32, unk1: KObject, unk2: [u8; 0x110]) -> Result<Session> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn Unknown3(&self, unk0: [u8; 0x110]) -> Result<u32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IHardwareOpusDecoderManager {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoderManager {
		IHardwareOpusDecoderManager(Session::from_kobject(obj))
	}
}
