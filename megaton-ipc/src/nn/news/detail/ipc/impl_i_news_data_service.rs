
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INewsDataService(Session);

impl AsRef<Session> for INewsDataService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INewsDataService {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown3(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for INewsDataService {
	unsafe fn from_kobject(obj: KObject) -> INewsDataService {
		INewsDataService(Session::from_kobject(obj))
	}
}
