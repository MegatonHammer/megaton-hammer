
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAudioRendererManager(Session);

impl IAudioRendererManager {
	// fn OpenAudioRenderer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetAudioRendererWorkBufferSize(&self, unk0: [u8; 0x34]) -> Result<u64> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetAudioRenderersProcessMasterVolume(&self, unk0: u64) -> Result<Session> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	// fn SetAudioRenderersProcessMasterVolume(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioRendererManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioRendererManager {
		IAudioRendererManager(Session::from_kobject(obj))
	}
}
