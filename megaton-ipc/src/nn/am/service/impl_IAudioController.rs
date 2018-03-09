
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAudioController(Session);

impl AsRef<Session> for IAudioController {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAudioController {
	pub fn SetExpectedMasterVolume(&self, unk0: f32, unk1: f32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: f32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetMainAppletExpectedMasterVolume(&self, ) -> Result<f32> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetLibraryAppletExpectedMasterVolume(&self, ) -> Result<f32> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<f32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ChangeMainAppletMasterVolume(&self, unk0: f32, unk1: i64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: f32,
			unk1: i64,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetTransparentVolumeRate(&self, unk0: f32) -> Result<()> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAudioController {
	unsafe fn from_kobject(obj: KObject) -> IAudioController {
		IAudioController(Session::from_kobject(obj))
	}
}
