
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IPrivateService(Session);

impl IPrivateService {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IPrivateService {
	unsafe fn from_kobject(obj: KObject) -> IPrivateService {
		IPrivateService(Session::from_kobject(obj))
	}
}
