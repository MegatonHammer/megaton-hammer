
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IAudioOut(Session);

impl IAudioOut {
	pub fn GetAudioOutState(&self, ) -> Result<(u32)> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn StartAudioOut(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn StopAudioOut(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn AppendAudioOutBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RegisterBufferEvent(&self, ) -> Result<(KObject)> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn GetReleasedAudioOutBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ContainsAudioOutBuffer(&self, unk0: u64) -> Result<(u8)> {
		let req = Request::new(6)
			.args(unk0)
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn AppendAudioOutBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetReleasedAudioOutBuffer(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IAudioOut {
	unsafe fn from_kobject(obj: KObject) -> IAudioOut {
		IAudioOut(Session::from_kobject(obj))
	}
}
