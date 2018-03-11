
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IHardwareOpusDecoder(Session);

impl AsRef<Session> for IHardwareOpusDecoder {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IHardwareOpusDecoder {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IHardwareOpusDecoder {
	unsafe fn from_kobject(obj: KObject) -> IHardwareOpusDecoder {
		IHardwareOpusDecoder(Session::from_kobject(obj))
	}
}
