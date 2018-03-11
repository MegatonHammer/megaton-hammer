
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IDsEndpoint(Session);

impl AsRef<Session> for IDsEndpoint {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDsEndpoint {
	pub fn PostBufferAsync(&self, size: u32, buffer: u64) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown1(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetCompletionEvent(&self, ) -> Result<KObject> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetReportData(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Stall(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown5(&self, unk0: u8) -> Result<()> {
		let req = Request::new(5)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDsEndpoint {
	unsafe fn from_kobject(obj: KObject) -> IDsEndpoint {
		IDsEndpoint(Session::from_kobject(obj))
	}
}
