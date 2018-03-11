
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ILibraryAppletCreator(Session);

impl AsRef<Session> for ILibraryAppletCreator {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ILibraryAppletCreator {
	pub fn create_library_applet(&self, unk0: u32, unk1: u32) -> Result<::nn::am::service::ILibraryAppletAccessor> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn terminate_all_library_applets(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn are_any_library_applets_left(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn create_storage(&self, unk0: i64) -> Result<::nn::am::service::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_transfer_memory_storage(&self, unk0: bool, unk1: i64, unk2: &KObject) -> Result<::nn::am::service::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: i64,
		}
		let req = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
			})
			.copy_handle(unk2)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn create_handle_storage(&self, unk0: i64, unk1: &KObject) -> Result<::nn::am::service::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(unk0)
			.copy_handle(unk1)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

}

impl FromKObject for ILibraryAppletCreator {
	unsafe fn from_kobject(obj: KObject) -> ILibraryAppletCreator {
		ILibraryAppletCreator(Session::from_kobject(obj))
	}
}
