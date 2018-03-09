
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IAlbumControlService(Session);

impl IAlbumControlService {
	pub fn get_service() -> Result<IAlbumControlService> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"caps:c\0\0").map(|s| unsafe { IAlbumControlService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAlbumControlService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAlbumControlService {
	pub fn Unknown2001(&self, unk0: u8) -> Result<()> {
		let req = Request::new(2001)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2002(&self, unk0: u8) -> Result<()> {
		let req = Request::new(2002)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2011(&self, unk0: u64, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(2011)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2012(&self, unk0: u64, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(2012)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2013(&self, unk0: u64) -> Result<u64> {
		let req = Request::new(2013)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn Unknown2014(&self, unk0: u64) -> Result<()> {
		let req = Request::new(2014)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2101(&self, ) -> Result<()> {
		let req = Request::new(2101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2102(&self, ) -> Result<()> {
		let req = Request::new(2102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2201(&self, ) -> Result<()> {
		let req = Request::new(2201)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown2301(&self, ) -> Result<()> {
		let req = Request::new(2301)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAlbumControlService {
	unsafe fn from_kobject(obj: KObject) -> IAlbumControlService {
		IAlbumControlService(Session::from_kobject(obj))
	}
}
