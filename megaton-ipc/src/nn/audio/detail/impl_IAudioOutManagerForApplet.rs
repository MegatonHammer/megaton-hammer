
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioOutManagerForApplet(Session);

impl IAudioOutManagerForApplet {
	pub fn get_service() -> Result<IAudioOutManagerForApplet> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audout:a").map(|s| unsafe { IAudioOutManagerForApplet::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IAudioOutManagerForApplet {
	pub fn RequestSuspend(&self, unk0: u64, unk1: u64) -> Result<KObject> {
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

	pub fn RequestResume(&self, unk0: u64, unk1: u64) -> Result<KObject> {
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

impl FromKObject for IAudioOutManagerForApplet {
	unsafe fn from_kobject(obj: KObject) -> IAudioOutManagerForApplet {
		IAudioOutManagerForApplet(Session::from_kobject(obj))
	}
}
