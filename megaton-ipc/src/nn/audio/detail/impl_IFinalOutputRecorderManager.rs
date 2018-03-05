
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IFinalOutputRecorderManager(Session);

impl IFinalOutputRecorderManager {
	// fn OpenFinalOutputRecorder(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFinalOutputRecorderManager {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorderManager {
		IFinalOutputRecorderManager(Session::from_kobject(obj))
	}
}
