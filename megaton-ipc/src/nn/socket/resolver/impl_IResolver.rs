
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IResolver(Session);

impl IResolver {
	pub fn get_service() -> Result<IResolver> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"sfdnsres").map(|s| unsafe { IResolver::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IResolver {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IResolver {
	// fn SetDnsAddressesPrivate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetDnsAddressPrivate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetHostByName(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetHostByAddr(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetHostStringError(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetGaiStringError(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetAddrInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetNameInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn RequestCancelHandle(&self, unk0: u64) -> Result<u32> {
		let req = Request::new(8)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn CancelSocketCall(&self, unk0: u32, unk1: u64) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(9)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown10(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Unknown11(&self, ) -> Result<()> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IResolver {
	unsafe fn from_kobject(obj: KObject) -> IResolver {
		IResolver(Session::from_kobject(obj))
	}
}
