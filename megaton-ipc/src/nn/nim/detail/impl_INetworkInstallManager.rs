
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct INetworkInstallManager(Session);

impl INetworkInstallManager {
	pub fn get_service() -> Result<INetworkInstallManager> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"nim\0\0\0\0\0").map(|s| unsafe { INetworkInstallManager::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for INetworkInstallManager {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl INetworkInstallManager {
	pub fn Unknown1(&self, unk0: u128) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn Unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INetworkInstallManager {
	unsafe fn from_kobject(obj: KObject) -> INetworkInstallManager {
		INetworkInstallManager(Session::from_kobject(obj))
	}
}
