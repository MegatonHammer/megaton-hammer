
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IFinalOutputRecorderManagerForApplet(Session);

impl IFinalOutputRecorderManagerForApplet {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFinalOutputRecorderManagerForApplet {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorderManagerForApplet {
		IFinalOutputRecorderManagerForApplet(Session::from_kobject(obj))
	}
}
