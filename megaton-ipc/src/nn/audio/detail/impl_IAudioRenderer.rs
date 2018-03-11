
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IAudioRenderer(Session);

impl AsRef<Session> for IAudioRenderer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioRenderer {
	pub fn GetAudioRendererSampleRate(&self, ) -> Result<u32> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAudioRendererSampleCount(&self, ) -> Result<u32> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAudioRendererMixBufferCount(&self, ) -> Result<u32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAudioRendererState(&self, ) -> Result<u32> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn RequestUpdateAudioRenderer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn StartAudioRenderer(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopAudioRenderer(&self, ) -> Result<()> {
		let req = Request::new(6)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn QuerySystemEvent(&self, ) -> Result<KObject> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn SetAudioRendererRenderingTimeLimit(&self, unk0: u32) -> Result<()> {
		let req = Request::new(8)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetAudioRendererRenderingTimeLimit(&self, ) -> Result<u32> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IAudioRenderer {
	unsafe fn from_kobject(obj: KObject) -> IAudioRenderer {
		IAudioRenderer(Session::from_kobject(obj))
	}
}
