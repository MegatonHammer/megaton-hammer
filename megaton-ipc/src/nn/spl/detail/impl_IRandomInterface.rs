
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IRandomInterface(Session);

impl IRandomInterface {
	// fn GetRandomBytes(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IRandomInterface {
	unsafe fn from_kobject(obj: KObject) -> IRandomInterface {
		IRandomInterface(Session::from_kobject(obj))
	}
}
