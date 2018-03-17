
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAudioInManagerForDebugger(Session);

impl IAudioInManagerForDebugger {
	pub fn new() -> Result<IAudioInManagerForDebugger> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"audin:d\0").map(|s| unsafe { IAudioInManagerForDebugger::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAudioInManagerForDebugger {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioInManagerForDebugger {
	pub fn list_audio_ins(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_audio_in(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn list_audio_ins_ex(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn open_audio_in_ex(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn list_audio_ins_ex_and_probe(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioInManagerForDebugger {
	unsafe fn from_kobject(obj: KObject) -> IAudioInManagerForDebugger {
		IAudioInManagerForDebugger(Session::from_kobject(obj))
	}
}
