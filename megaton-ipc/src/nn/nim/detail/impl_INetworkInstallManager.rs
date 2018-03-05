
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct INetworkInstallManager(Session);

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
