
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IRequest(Session);

impl IRequest {
	pub fn get_service() -> Result<IRequest> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"eupld:r\0").map(|s| unsafe { IRequest::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IRequest {
	pub fn Unknown0(&self, ) -> Result<KObject> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn Unknown1(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Unknown3(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Unknown4(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown5(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IRequest {
	unsafe fn from_kobject(obj: KObject) -> IRequest {
		IRequest(Session::from_kobject(obj))
	}
}
