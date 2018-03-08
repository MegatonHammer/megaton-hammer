
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
		let r = sm.GetService(*b"mii:e\0\0\0").map(|s| unsafe { IStaticService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"mii:u\0\0\0").map(|s| unsafe { IStaticService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IStaticService {
	pub fn GetDatabaseServiceSharedPointer(&self, unk0: i32) -> Result<::nn::mii::detail::IDatabaseService> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for IStaticService {
	unsafe fn from_kobject(obj: KObject) -> IStaticService {
		IStaticService(Session::from_kobject(obj))
	}
}
