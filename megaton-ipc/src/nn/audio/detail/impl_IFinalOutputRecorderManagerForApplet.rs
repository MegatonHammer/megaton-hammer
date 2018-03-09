
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IFinalOutputRecorderManagerForApplet(Session);

impl IFinalOutputRecorderManagerForApplet {
	pub fn get_service() -> Result<IFinalOutputRecorderManagerForApplet> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audrec:a").map(|s| unsafe { IFinalOutputRecorderManagerForApplet::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFinalOutputRecorderManagerForApplet {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFinalOutputRecorderManagerForApplet {
	pub fn Unknown0(&self, unk0: u64, unk1: u64) -> Result<KObject> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown1(&self, unk0: u64, unk1: u64) -> Result<KObject> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IFinalOutputRecorderManagerForApplet {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorderManagerForApplet {
		IFinalOutputRecorderManagerForApplet(Session::from_kobject(obj))
	}
}
