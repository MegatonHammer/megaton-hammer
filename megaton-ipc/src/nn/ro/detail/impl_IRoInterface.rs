
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IRoInterface(Session);

impl IRoInterface {
	pub fn get_service() -> Result<IRoInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ldr:ro\0\0").map(|s| unsafe { IRoInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IRoInterface {
	pub fn Unknown0(&self, unk0: u64, unk1: u64, unk2: u64, unk3: u64, unk4: u64) -> Result<u64> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
			unk2: u64,
			unk3: u64,
			unk4: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
				unk4,
			})
			.send_pid()
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown1(&self, unk0: u64, unk1: u64) -> Result<()> {
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
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2(&self, unk0: u64, unk1: u64, unk2: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
			unk2: u64,
		}
		let req = Request::new(2)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown3(&self, unk0: u64, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown4(&self, unk0: u64, unk2: KObject) -> Result<()> {
		let req = Request::new(4)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IRoInterface {
	unsafe fn from_kobject(obj: KObject) -> IRoInterface {
		IRoInterface(Session::from_kobject(obj))
	}
}
