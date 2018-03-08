
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IStaticService(Session);

impl IStaticService {
	pub fn get_service() -> Result<IStaticService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"ntc\0\0\0\0\0").map(|s| unsafe { IStaticService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IStaticService {
	pub fn Unknown0(&self, unk0: u32, unk1: u32) -> Result<Session> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
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

	pub fn Unknown100(&self, ) -> Result<()> {
		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown101(&self, ) -> Result<()> {
		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IStaticService {
	unsafe fn from_kobject(obj: KObject) -> IStaticService {
		IStaticService(Session::from_kobject(obj))
	}
}
