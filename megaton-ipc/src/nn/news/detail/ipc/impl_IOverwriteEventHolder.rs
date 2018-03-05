
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IOverwriteEventHolder(Session);

impl IOverwriteEventHolder {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IOverwriteEventHolder {
	unsafe fn from_kobject(obj: KObject) -> IOverwriteEventHolder {
		IOverwriteEventHolder(Session::from_kobject(obj))
	}
}
