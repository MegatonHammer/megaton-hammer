
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IUserInterface(Session);

impl IUserInterface {
	pub fn get_service() -> Result<IUserInterface> {
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

impl IUserInterface {
	pub fn Initialize(&self, reserved: u64) -> Result<()> {
		let req = Request::new(0)
			.args(reserved)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetService(&self, name: ::ServiceName) -> Result<KObject> {
		let req = Request::new(1)
			.args(name)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn RegisterService(&self, name: ::ServiceName, unk1: u8, maxHandles: u32) -> Result<KObject> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			name: ::ServiceName,
			unk1: u8,
			maxHandles: u32,
		}
		let req = Request::new(2)
			.args(InRaw {
				name,
				unk1,
				maxHandles,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn UnregisterService(&self, name: ::ServiceName) -> Result<()> {
		let req = Request::new(3)
			.args(name)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IUserInterface {
	unsafe fn from_kobject(obj: KObject) -> IUserInterface {
		IUserInterface(Session::from_kobject(obj))
	}
}
