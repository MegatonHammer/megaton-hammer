
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IAudioInManagerForApplet(Session);

impl IAudioInManagerForApplet {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown2(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn Unknown3(&self, unk0: u32, unk1: u64, unk2: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
			unk2: u64,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
}

impl FromKObject for IAudioInManagerForApplet {
	unsafe fn from_kobject(obj: KObject) -> IAudioInManagerForApplet {
		IAudioInManagerForApplet(Session::from_kobject(obj))
	}
}
