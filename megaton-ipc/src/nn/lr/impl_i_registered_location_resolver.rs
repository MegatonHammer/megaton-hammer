
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IRegisteredLocationResolver(Session);

impl AsRef<Session> for IRegisteredLocationResolver {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IRegisteredLocationResolver {
	// fn get_patch_type0_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn register_patch_type0_fallback_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unregister_patch_type0_fallback_path(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_patch_type0_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_patch_type1_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn register_patch_type1_fallback_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn unregister_patch_type1_fallback_path(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_patch_type1_nca_path(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IRegisteredLocationResolver {
	unsafe fn from_kobject(obj: KObject) -> IRegisteredLocationResolver {
		IRegisteredLocationResolver(Session::from_kobject(obj))
	}
}
