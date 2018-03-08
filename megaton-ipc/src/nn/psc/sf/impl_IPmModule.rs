
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IPmModule(Session);

impl IPmModule {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IPmModule {
	unsafe fn from_kobject(obj: KObject) -> IPmModule {
		IPmModule(Session::from_kobject(obj))
	}
}
