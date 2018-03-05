
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAudioInManager(Session);

impl IAudioInManager {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioInManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioInManager {
		IAudioInManager(Session::from_kobject(obj))
	}
}
