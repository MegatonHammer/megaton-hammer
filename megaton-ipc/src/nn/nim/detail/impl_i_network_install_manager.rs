
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct INetworkInstallManager(Session);

impl INetworkInstallManager {
	pub fn new() -> Result<INetworkInstallManager> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"nim\0\0\0\0\0").map(|s| unsafe { INetworkInstallManager::from_kobject(s) });
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
	pub fn unknown1(&self, unk0: u128) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for INetworkInstallManager {
	unsafe fn from_kobject(obj: KObject) -> INetworkInstallManager {
		INetworkInstallManager(Session::from_kobject(obj))
	}
}
