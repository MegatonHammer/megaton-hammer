
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct INotifier(Session);

impl INotifier {
	// fn GetSystemEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INotifier {
	unsafe fn from_kobject(obj: KObject) -> INotifier {
		INotifier(Session::from_kobject(obj))
	}
}
