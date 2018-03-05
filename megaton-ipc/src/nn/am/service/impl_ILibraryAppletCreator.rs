
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ILibraryAppletCreator(Session);

impl ILibraryAppletCreator {
	pub fn CreateLibraryApplet(&self, unk0: u32, unk1: u32) -> Result<::nn::am::service::ILibraryAppletAccessor> {
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
	pub fn TerminateAllLibraryApplets(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn AreAnyLibraryAppletsLeft(&self, ) -> Result<bool> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn CreateStorage(&self, unk0: i64) -> Result<::nn::am::service::IStorage> {
		let req = Request::new(10)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}
	// fn CreateTransferMemoryStorage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn CreateHandleStorage(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ILibraryAppletCreator {
	unsafe fn from_kobject(obj: KObject) -> ILibraryAppletCreator {
		ILibraryAppletCreator(Session::from_kobject(obj))
	}
}
