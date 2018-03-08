
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioOutManager(Session);

impl IAudioOutManager {
	// fn ListAudioOuts(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn OpenAudioOut(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioOutManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioOutManager {
		IAudioOutManager(Session::from_kobject(obj))
	}
}
