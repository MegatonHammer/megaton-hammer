
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IAddOnContentManager(Session);

impl IAddOnContentManager {
	pub fn get_service() -> Result<IAddOnContentManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"aoc:u\0\0\0").map(|s| unsafe { IAddOnContentManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IAddOnContentManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IAddOnContentManager {
	pub fn CountAddOnContentByApplicationId(&self, unk0: ::nn::ncm::ApplicationId) -> Result<i32> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn ListAddOnContentByApplicationId(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn CountAddOnContent(&self, unk0: u64) -> Result<i32> {
		let req = Request::new(2)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn ListAddOnContent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetAddOnContentBaseIdByApplicationId(&self, unk0: ::nn::ncm::ApplicationId) -> Result<u64> {
		let req = Request::new(4)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAddOnContentBaseId(&self, unk0: u64) -> Result<u64> {
		let req = Request::new(5)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn PrepareAddOnContentByApplicationId(&self, unk0: i32, unk1: ::nn::ncm::ApplicationId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: ::nn::ncm::ApplicationId,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn PrepareAddOnContent(&self, unk0: i32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: u64,
		}
		let req = Request::new(7)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IAddOnContentManager {
	unsafe fn from_kobject(obj: KObject) -> IAddOnContentManager {
		IAddOnContentManager(Session::from_kobject(obj))
	}
}
