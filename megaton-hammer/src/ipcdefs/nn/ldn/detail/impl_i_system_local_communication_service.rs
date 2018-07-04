
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISystemLocalCommunicationService<T>(T);

impl ISystemLocalCommunicationService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISystemLocalCommunicationService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystemLocalCommunicationService(domain)),
			Err((sess, err)) => Err((ISystemLocalCommunicationService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystemLocalCommunicationService<Session>> {
		Ok(ISystemLocalCommunicationService(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystemLocalCommunicationService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystemLocalCommunicationService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystemLocalCommunicationService<T> {
	pub fn get_state(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_network_info(&self, unk0: &mut [u8; 0x480]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ipv4_address(&self, ) -> Result<(u32, u32)> {
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

	pub fn get_disconnect_reason(&self, ) -> Result<u16> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u16> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_security_parameter(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_network_config(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn attach_state_change_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_network_info_latest_update(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn scan(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn scan_private(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn open_access_point(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(200)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn close_access_point(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(201)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_network(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_network_private(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn destroy_network(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(204)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn reject(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(205)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_advertise_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_station_accept_policy(&self, unk0: u8) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(207)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn add_accept_filter_entry(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn clear_accept_filter(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(209)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_station(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(300)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn close_station(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(301)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn connect(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn connect_private(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn disconnect(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(304)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn initialize_system(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(400)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn finalize_system(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(401)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISystemLocalCommunicationService<T> {
	fn from(obj: T) -> ISystemLocalCommunicationService<T> {
		ISystemLocalCommunicationService(obj)
	}
}
