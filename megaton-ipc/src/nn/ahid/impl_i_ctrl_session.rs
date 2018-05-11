
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ICtrlSession<T>(T);

impl ICtrlSession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ICtrlSession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ICtrlSession(domain)),
			Err((sess, err)) => Err((ICtrlSession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ICtrlSession<Session>> {
		Ok(ICtrlSession(self.0.duplicate()?))
	}
}

impl<T> Deref for ICtrlSession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ICtrlSession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ICtrlSession<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown4(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown5(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown6(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown7(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown8(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown9(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown10(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown11(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown12(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ICtrlSession<T> {
	fn from(obj: T) -> ICtrlSession<T> {
		ICtrlSession(obj)
	}
}
