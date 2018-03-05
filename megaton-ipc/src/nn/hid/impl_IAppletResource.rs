
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAppletResource(Session);

impl IAppletResource {
	// fn GetSharedMemoryHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAppletResource {
	unsafe fn from_kobject(obj: KObject) -> IAppletResource {
		IAppletResource(Session::from_kobject(obj))
	}
}
