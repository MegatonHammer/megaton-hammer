
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDebugFunctions<T>(T);

impl IDebugFunctions<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDebugFunctions<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDebugFunctions(domain)),
			Err((sess, err)) => Err((IDebugFunctions(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDebugFunctions<Session>> {
		Ok(IDebugFunctions(self.0.duplicate()?))
	}
}

impl<T> Deref for IDebugFunctions<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDebugFunctions<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDebugFunctions<T> {
	pub fn notify_message_to_home_menu_for_debug(&self, unk0: ::ipcdefs::nn::am::AppletMessage) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_main_application(&self, ) -> Result<::ipcdefs::nn::am::service::IApplicationAccessor<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn emulate_button_event(&self, unk0: ::ipcdefs::nn::am::service::EmulatedButtonEvent) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn invalidate_transition_layer(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDebugFunctions<T> {
	fn from(obj: T) -> IDebugFunctions<T> {
		IDebugFunctions(obj)
	}
}
