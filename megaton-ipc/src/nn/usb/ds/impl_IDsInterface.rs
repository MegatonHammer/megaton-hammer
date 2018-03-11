
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IDsInterface(Session);

impl AsRef<Session> for IDsInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDsInterface {
	// fn GetDsEndpoint(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetSetupEvent(&self, ) -> Result<KObject> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn EnableInterface(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DisableInterface(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CtrlInPostBufferAsync(&self, size: u32, buffer: u64) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req = Request::new(5)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CtrlOutPostBufferAsync(&self, size: u32, buffer: u64) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req = Request::new(6)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetCtrlInCompletionEvent(&self, ) -> Result<KObject> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetCtrlInReportData(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetCtrlOutCompletionEvent(&self, ) -> Result<KObject> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetCtrlOutReportData(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn StallCtrl(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDsInterface {
	unsafe fn from_kobject(obj: KObject) -> IDsInterface {
		IDsInterface(Session::from_kobject(obj))
	}
}
