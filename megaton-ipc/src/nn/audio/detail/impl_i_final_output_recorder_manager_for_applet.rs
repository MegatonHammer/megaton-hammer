
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IFinalOutputRecorderManagerForApplet(Session);

impl IFinalOutputRecorderManagerForApplet {
	pub fn new() -> Result<IFinalOutputRecorderManagerForApplet> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"audrec:a").map(|s| unsafe { IFinalOutputRecorderManagerForApplet::from_kobject(s) });
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
	pub fn request_suspend_final_output_recorders(&self, unk0: u64, unk1: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

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

	pub fn request_resume_final_output_recorders(&self, unk0: u64, unk1: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

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
