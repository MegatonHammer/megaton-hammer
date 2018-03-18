
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IReadSession(Session);

impl AsRef<Session> for IReadSession {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IReadSession {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IReadSession {
	unsafe fn from_kobject(obj: KObject) -> IReadSession {
		IReadSession(Session::from_kobject(obj))
	}
}
