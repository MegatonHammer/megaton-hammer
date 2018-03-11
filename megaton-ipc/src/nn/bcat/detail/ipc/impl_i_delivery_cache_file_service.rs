
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDeliveryCacheFileService(Session);

impl AsRef<Session> for IDeliveryCacheFileService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeliveryCacheFileService {
	pub fn open(&self, unk0: ::nn::bcat::DirectoryName, unk1: ::nn::bcat::FileName) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::bcat::DirectoryName,
			unk1: ::nn::bcat::FileName,
		}
		let req = Request::new(0)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_size(&self, ) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_digest(&self, ) -> Result<::nn::bcat::Digest> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<::nn::bcat::Digest> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IDeliveryCacheFileService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheFileService {
		IDeliveryCacheFileService(Session::from_kobject(obj))
	}
}
