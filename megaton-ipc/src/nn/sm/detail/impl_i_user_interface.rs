
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IUserInterface(Session);

impl IUserInterface {
	pub fn new() -> Result<IUserInterface> {
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let mut session = 0;
		let r = unsafe { svc::connect_to_named_port(&mut session, "sm:".as_ptr()) };
		if r != 0 {
			return Err(Error(r))
		} else {
			return Ok(unsafe { IUserInterface::from_kobject(KObject::new(session)) });
		}
	}
}

impl AsRef<Session> for IUserInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IUserInterface {
	pub fn initialize(&self, reserved: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(reserved)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_service(&self, name: ::ServiceName) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(name)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn register_service(&self, name: ::ServiceName, unk1: u8, max_handles: u32) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			name: ::ServiceName,
			unk1: u8,
			max_handles: u32,
		}
		let req = Request::new(2)
			.args(InRaw {
				name,
				unk1,
				max_handles,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unregister_service(&self, name: ::ServiceName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(name)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IUserInterface {
	unsafe fn from_kobject(obj: KObject) -> IUserInterface {
		IUserInterface(Session::from_kobject(obj))
	}
}
