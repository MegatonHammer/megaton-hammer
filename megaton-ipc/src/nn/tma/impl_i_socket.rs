
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISocket(Session);

impl AsRef<Session> for ISocket {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISocket {
	pub fn close(&self, ) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	// fn connect(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn bind(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn listen(&self, unk0: u32) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	// fn accept(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn recv(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn shutdown(&self, unk0: u32) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u32,
			unk2: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn fcntl(&self, unk0: u32, unk1: u32) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(8)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: u32,
			unk3: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

}

impl FromKObject for ISocket {
	unsafe fn from_kobject(obj: KObject) -> ISocket {
		ISocket(Session::from_kobject(obj))
	}
}
