
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IOverlayAppletProxy<T>(T);

impl IOverlayAppletProxy<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IOverlayAppletProxy<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IOverlayAppletProxy(domain)),
			Err((sess, err)) => Err((IOverlayAppletProxy(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IOverlayAppletProxy<Session>> {
		Ok(IOverlayAppletProxy(self.0.duplicate()?))
	}
}

impl<T> Deref for IOverlayAppletProxy<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IOverlayAppletProxy<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IOverlayAppletProxy<T> {
	pub fn get_common_state_getter(&self, ) -> Result<::ipcdefs::nn::am::service::ICommonStateGetter<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_self_controller(&self, ) -> Result<::ipcdefs::nn::am::service::ISelfController<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_window_controller(&self, ) -> Result<::ipcdefs::nn::am::service::IWindowController<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_audio_controller(&self, ) -> Result<::ipcdefs::nn::am::service::IAudioController<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_display_controller(&self, ) -> Result<::ipcdefs::nn::am::service::IDisplayController<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_process_winding_controller(&self, ) -> Result<::ipcdefs::nn::am::service::IProcessWindingController<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_library_applet_creator(&self, ) -> Result<::ipcdefs::nn::am::service::ILibraryAppletCreator<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_overlay_functions(&self, ) -> Result<::ipcdefs::nn::am::service::IOverlayFunctions<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_debug_functions(&self, ) -> Result<::ipcdefs::nn::am::service::IDebugFunctions<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1000)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

}

impl<T: Object> From<T> for IOverlayAppletProxy<T> {
	fn from(obj: T) -> IOverlayAppletProxy<T> {
		IOverlayAppletProxy(obj)
	}
}
