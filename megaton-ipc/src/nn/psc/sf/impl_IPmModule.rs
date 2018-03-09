
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IPmModule(Session);

impl AsRef<Session> for IPmModule {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IPmModule {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IPmModule {
	unsafe fn from_kobject(obj: KObject) -> IPmModule {
		IPmModule(Session::from_kobject(obj))
	}
}
