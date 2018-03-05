
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IScreenShotApplicationService(Session);

impl IScreenShotApplicationService {
	// fn SaveScreenShot(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SaveScreenShotEx0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IScreenShotApplicationService {
	unsafe fn from_kobject(obj: KObject) -> IScreenShotApplicationService {
		IScreenShotApplicationService(Session::from_kobject(obj))
	}
}
