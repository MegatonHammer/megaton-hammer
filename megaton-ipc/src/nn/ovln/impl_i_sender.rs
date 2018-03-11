
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISender(Session);

impl ISender {
	pub fn new() -> Result<ISender> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"ovln:snd").map(|s| unsafe { ISender::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for ISender {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISender {
	pub fn unknown0(&self, unk1: u64, unk2: u64, unk3: u64, unk4: u64, unk5: u64, unk6: u64, unk7: u64, unk8: u64, unk9: u64, unk10: u64, unk11: u64, unk12: u64, unk13: u64, unk14: u64, unk15: u64, unk16: u64, unk17: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk1: u64,
			unk2: u64,
			unk3: u64,
			unk4: u64,
			unk5: u64,
			unk6: u64,
			unk7: u64,
			unk8: u64,
			unk9: u64,
			unk10: u64,
			unk11: u64,
			unk12: u64,
			unk13: u64,
			unk14: u64,
			unk15: u64,
			unk16: u64,
			unk17: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk1,
				unk2,
				unk3,
				unk4,
				unk5,
				unk6,
				unk7,
				unk8,
				unk9,
				unk10,
				unk11,
				unk12,
				unk13,
				unk14,
				unk15,
				unk16,
				unk17,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for ISender {
	unsafe fn from_kobject(obj: KObject) -> ISender {
		ISender(Session::from_kobject(obj))
	}
}
