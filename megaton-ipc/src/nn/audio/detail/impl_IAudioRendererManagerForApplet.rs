
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioRendererManagerForApplet(Session);

impl IAudioRendererManagerForApplet {
	pub fn get_service() -> Result<IAudioRendererManagerForApplet> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audren:a").map(|s| unsafe { IAudioRendererManagerForApplet::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IAudioRendererManagerForApplet {
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

	pub fn Unknown2(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown3(&self, unk0: u32, unk1: u64, unk2: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
			unk2: u64,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown4(&self, unk0: u64) -> Result<()> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown5(&self, unk0: u64) -> Result<()> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioRendererManagerForApplet {
	unsafe fn from_kobject(obj: KObject) -> IAudioRendererManagerForApplet {
		IAudioRendererManagerForApplet(Session::from_kobject(obj))
	}
}
