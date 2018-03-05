
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct LocationResolverInterface(Session);

impl LocationResolverInterface {
}

impl FromKObject for LocationResolverInterface {
	unsafe fn from_kobject(obj: KObject) -> LocationResolverInterface {
		LocationResolverInterface(Session::from_kobject(obj))
	}
}
