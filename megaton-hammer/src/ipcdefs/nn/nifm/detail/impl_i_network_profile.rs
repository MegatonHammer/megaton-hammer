
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct INetworkProfile<T>(T);

impl INetworkProfile<Session> {
	pub fn to_domain(self) -> ::core::result::Result<INetworkProfile<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INetworkProfile(domain)),
			Err((sess, err)) => Err((INetworkProfile(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INetworkProfile<Session>> {
		Ok(INetworkProfile(self.0.duplicate()?))
	}
}

impl<T> Deref for INetworkProfile<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INetworkProfile<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INetworkProfile<T> {
	pub fn update(&self, unk0: &::ipcdefs::nn::nifm::detail::sf::NetworkProfileData) -> Result<::ipcdefs::nn::util::Uuid> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_ref(unk0, 0x19))
			;
		let res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(not(feature = "switch-3.0.0"))]
	pub fn persist(&self, unk0: ::ipcdefs::nn::util::Uuid) -> Result<::ipcdefs::nn::util::Uuid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn persist_old(&self, unk0: ::ipcdefs::nn::util::Uuid) -> Result<::ipcdefs::nn::util::Uuid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn persist(&self, ) -> Result<::ipcdefs::nn::util::Uuid> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<::ipcdefs::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for INetworkProfile<T> {
	fn from(obj: T) -> INetworkProfile<T> {
		INetworkProfile(obj)
	}
}
