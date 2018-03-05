
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct INewsDataService(Session);

impl INewsDataService {
	// fn Unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown1(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown3(&self, ) -> Result<u64> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
}

impl FromKObject for INewsDataService {
	unsafe fn from_kobject(obj: KObject) -> INewsDataService {
		INewsDataService(Session::from_kobject(obj))
	}
}
