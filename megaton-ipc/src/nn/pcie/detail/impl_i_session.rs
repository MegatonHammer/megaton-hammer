
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISession<T>(T);

impl ISession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISession(domain)),
			Err((sess, err)) => Err((ISession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISession<Session>> {
		Ok(ISession(self.0.duplicate()?))
	}
}

impl<T> Deref for ISession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISession<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown2(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

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
	// fn unknown13(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown14(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown16(&self, unk0: u8, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u32,
		}
		let req = Request::new(16)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown17(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown18(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(18)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown19(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown20(&self, unk0: u8, unk1: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u32,
		}
		let req = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISession<T> {
	fn from(obj: T) -> ISession<T> {
		ISession(obj)
	}
}
