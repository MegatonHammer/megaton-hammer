
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDeliveryCacheProgressService<T>(T);

impl IDeliveryCacheProgressService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDeliveryCacheProgressService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDeliveryCacheProgressService(domain)),
			Err((sess, err)) => Err((IDeliveryCacheProgressService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDeliveryCacheProgressService<Session>> {
		Ok(IDeliveryCacheProgressService(self.0.duplicate()?))
	}
}

impl<T> Deref for IDeliveryCacheProgressService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDeliveryCacheProgressService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDeliveryCacheProgressService<T> {
	pub fn get_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_impl(&self, unk0: &mut ::ipcdefs::nn::bcat::detail::DeliveryCacheProgressImpl) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDeliveryCacheProgressService<T> {
	fn from(obj: T) -> IDeliveryCacheProgressService<T> {
		IDeliveryCacheProgressService(obj)
	}
}
