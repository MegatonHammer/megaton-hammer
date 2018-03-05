
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IService(Session);

impl IService {
	pub fn Unknown0(&self, unk0: u64, unk1: u64) -> Result<()> {
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
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
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
	pub fn TransitionToFatalError(&self, errorCode: u64, unk1: u64, errorBuf: [u8; 0x110]) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			errorCode: u64,
			unk1: u64,
		}
		let req = Request::new(2)
			.args(InRaw {
				errorCode,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IService {
	unsafe fn from_kobject(obj: KObject) -> IService {
		IService(Session::from_kobject(obj))
	}
}
