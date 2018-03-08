
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioRendererManager(Session);

impl IAudioRendererManager {
	pub fn get_service() -> Result<IAudioRendererManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"audren:u").map(|s| unsafe { IAudioRendererManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IAudioRendererManager {
	// fn OpenAudioRenderer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetAudioRendererWorkBufferSize(&self, UNKNOWN) -> Result<UNKNOWN>;
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
