
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISystem<T>(T);

impl ISystem<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISystem<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISystem(domain)),
			Err((sess, err)) => Err((ISystem(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISystem<Session>> {
		Ok(ISystem(self.0.duplicate()?))
	}
}

impl<T> Deref for ISystem<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISystem<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISystem<T> {
	// fn initialize(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn finalize(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_state_old(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn is_nfc_enabled_old(&self, ) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_nfc_enabled_old(&self, unk0: u8) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn initialize_system(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn finalize_system(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_nfc_enabled(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_devices(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_device_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_npad_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn attach_availability_change_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn start_detection(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn stop_detection(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_tag_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn attach_activate_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn attach_deactivate_event(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_nfc_enabled(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_mifare(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_mifare(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send_command_by_pass_through(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn keep_pass_through_session(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn release_pass_through_session(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ISystem<T> {
	fn from(obj: T) -> ISystem<T> {
		ISystem(obj)
	}
}
