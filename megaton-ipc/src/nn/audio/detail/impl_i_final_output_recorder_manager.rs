
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFinalOutputRecorderManager(Session);

impl IFinalOutputRecorderManager {
	pub fn new() -> Result<Arc<IFinalOutputRecorderManager>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFinalOutputRecorderManager>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"audrec:u").map(|s| Arc::new(unsafe { IFinalOutputRecorderManager::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFinalOutputRecorderManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFinalOutputRecorderManager {
	pub fn open_final_output_recorder(&self, unk0: u64, unk1: u64, unk2: &KObject) -> Result<(u128, Session)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			.copy_handle(unk2)
			;
		let mut res : Response<u128> = self.0.send(req)?;
		Ok((*res.get_raw(),unsafe { FromKObject::from_kobject(res.pop_handle()) }))
	}

}

impl FromKObject for IFinalOutputRecorderManager {
	unsafe fn from_kobject(obj: KObject) -> IFinalOutputRecorderManager {
		IFinalOutputRecorderManager(Session::from_kobject(obj))
	}
}
