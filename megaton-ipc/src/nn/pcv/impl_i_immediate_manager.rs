
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IImmediateManager(Session);

impl IImmediateManager {
	pub fn new() -> Result<IImmediateManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"pcv:imm\0").map(|s| unsafe { IImmediateManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IImmediateManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IImmediateManager {
	pub fn set_clock_rate(&self, unk0: i32, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u32,
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

}

impl FromKObject for IImmediateManager {
	unsafe fn from_kobject(obj: KObject) -> IImmediateManager {
		IImmediateManager(Session::from_kobject(obj))
	}
}
