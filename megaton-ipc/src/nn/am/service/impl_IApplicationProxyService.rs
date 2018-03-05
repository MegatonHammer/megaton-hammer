
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IApplicationProxyService(Session);

impl IApplicationProxyService {
	// fn OpenApplicationProxy(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IApplicationProxyService {
	unsafe fn from_kobject(obj: KObject) -> IApplicationProxyService {
		IApplicationProxyService(Session::from_kobject(obj))
	}
}
