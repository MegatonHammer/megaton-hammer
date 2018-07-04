
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IMonitorService<T>(T);

impl IMonitorService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IMonitorService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IMonitorService(domain)),
			Err((sess, err)) => Err((IMonitorService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IMonitorService<Session>> {
		Ok(IMonitorService(self.0.duplicate()?))
	}
}

impl<T> Deref for IMonitorService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IMonitorService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IMonitorService<T> {
	pub fn get_state_for_monitor(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_network_info_for_monitor(&self, unk0: &mut [u8; 0x480]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ipv4_address_for_monitor(&self, ) -> Result<(u32, u32)> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn get_disconnect_reason_for_monitor(&self, ) -> Result<u16> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u16> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_security_parameter_for_monitor(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_network_config_for_monitor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn initialize_monitor(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn finalize_monitor(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IMonitorService<T> {
	fn from(obj: T) -> IMonitorService<T> {
		IMonitorService(obj)
	}
}
