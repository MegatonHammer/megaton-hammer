
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IAudioRendererManager(Session);

impl IAudioRendererManager {
	pub fn new() -> Result<Arc<IAudioRendererManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IAudioRendererManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"audren:u") {
			let ret = Arc::new(IAudioRendererManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"audren:u").map(|s| Arc::new(unsafe { IAudioRendererManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
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
	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown4(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioRendererManager {
	unsafe fn from_kobject(obj: KObject) -> IAudioRendererManager {
		IAudioRendererManager(Session::from_kobject(obj))
	}
}
