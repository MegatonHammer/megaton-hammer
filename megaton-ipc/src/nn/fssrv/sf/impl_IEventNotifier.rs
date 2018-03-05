
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IEventNotifier(Session);

impl IEventNotifier {
	// fn BindEvent(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IEventNotifier {
	unsafe fn from_kobject(obj: KObject) -> IEventNotifier {
		IEventNotifier(Session::from_kobject(obj))
	}
}
