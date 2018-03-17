
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IAudioIn(Session);

impl AsRef<Session> for IAudioIn {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioIn {
	pub fn get_audio_in_state(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start_audio_in(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn stop_audio_in(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn append_audio_in_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn register_buffer_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_released_audio_in_buffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn contains_audio_in_buffer(&self, unk0: u64) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn append_audio_in_buffer_with_user_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn append_audio_in_buffer_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_released_audio_in_buffer_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn append_audio_in_buffer_with_user_event_ex(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown11(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown12(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn unknown13(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioIn {
	unsafe fn from_kobject(obj: KObject) -> IAudioIn {
		IAudioIn(Session::from_kobject(obj))
	}
}
