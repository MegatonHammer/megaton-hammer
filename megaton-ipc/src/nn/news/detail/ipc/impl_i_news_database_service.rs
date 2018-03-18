
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INewsDatabaseService(Session);

impl AsRef<Session> for INewsDatabaseService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INewsDatabaseService {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INewsDatabaseService {
	unsafe fn from_kobject(obj: KObject) -> INewsDatabaseService {
		INewsDatabaseService(Session::from_kobject(obj))
	}
}
