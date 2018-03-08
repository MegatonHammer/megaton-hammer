
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct INotifyService(Session);

impl INotifyService {
	pub fn get_service() -> Result<INotifyService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"pdm:ntfy").map(|s| unsafe { INotifyService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl INotifyService {
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
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2(&self, unk0: u8) -> Result<()> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown3(&self, unk0: u8) -> Result<()> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown4(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INotifyService {
	unsafe fn from_kobject(obj: KObject) -> INotifyService {
		INotifyService(Session::from_kobject(obj))
	}
}
