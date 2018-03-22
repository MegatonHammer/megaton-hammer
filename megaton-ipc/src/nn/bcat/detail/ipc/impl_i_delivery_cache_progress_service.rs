
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDeliveryCacheProgressService(Session);

impl AsRef<Session> for IDeliveryCacheProgressService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDeliveryCacheProgressService {
	pub fn get_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_impl(&self, unk0: &mut ::nn::bcat::detail::DeliveryCacheProgressImpl) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IDeliveryCacheProgressService {
	unsafe fn from_kobject(obj: KObject) -> IDeliveryCacheProgressService {
		IDeliveryCacheProgressService(Session::from_kobject(obj))
	}
}
