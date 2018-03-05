
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAudioOutManagerForApplet(Session);

impl IAudioOutManagerForApplet {
	// fn RequestSuspend(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RequestResume(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioOutManagerForApplet {
	unsafe fn from_kobject(obj: KObject) -> IAudioOutManagerForApplet {
		IAudioOutManagerForApplet(Session::from_kobject(obj))
	}
}
