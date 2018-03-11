
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IFinalOutputRecorder(Session);

impl AsRef<Session> for IFinalOutputRecorder {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFinalOutputRecorder {
	pub fn GetFinalOutputRecorderState(&self, ) -> Result<u32> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn StartFinalOutputRecorder(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopFinalOutputRecorder(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn AppendFinalOutputRecorderBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RegisterBufferEvent(&self, ) -> Result<KObject> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetReleasedFinalOutputRecorderBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ContainsFinalOutputRecorderBuffer(&self, unk0: u64) -> Result<u8> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IFinalOutputRecorder {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorder {
		IFinalOutputRecorder(Session::from_kobject(obj))
	}
}
