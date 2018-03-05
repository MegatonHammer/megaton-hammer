
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct INewlyArrivedEventHolder(Session);

impl INewlyArrivedEventHolder {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INewlyArrivedEventHolder {
	unsafe fn from_kobject(obj: KObject) -> INewlyArrivedEventHolder {
		INewlyArrivedEventHolder(Session::from_kobject(obj))
	}
}
