
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAudioRendererManager(Session);

impl IAudioRendererManager {
	pub fn new() -> Result<IAudioRendererManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"audren:u").map(|s| unsafe { IAudioRendererManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioRendererManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioRendererManager {
	// fn open_audio_renderer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_audio_renderer_work_buffer_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_audio_renderers_process_master_volume(&self, unk0: u64) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn set_audio_renderers_process_master_volume(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioRendererManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioRendererManager {
		IAudioRendererManager(Session::from_kobject(obj))
	}
}
