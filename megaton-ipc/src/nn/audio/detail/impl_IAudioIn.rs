
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IAudioIn(Session);

impl AsRef<Session> for IAudioIn {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioIn {
	pub fn GetAudioInState(&self, ) -> Result<u32> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn StartAudioIn(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopAudioIn(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn AppendAudioInBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RegisterBufferEvent(&self, ) -> Result<KObject> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetReleasedAudioInBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ContainsAudioInBuffer(&self, unk0: u64) -> Result<u8> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IAudioIn {
	unsafe fn from_kobject(obj: KObject) -> IAudioIn {
		IAudioIn(Session::from_kobject(obj))
	}
}
