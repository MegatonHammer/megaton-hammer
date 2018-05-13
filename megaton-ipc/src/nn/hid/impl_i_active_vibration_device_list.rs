
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IActiveVibrationDeviceList<T>(T);

impl IActiveVibrationDeviceList<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IActiveVibrationDeviceList<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IActiveVibrationDeviceList(domain)),
			Err((sess, err)) => Err((IActiveVibrationDeviceList(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IActiveVibrationDeviceList<Session>> {
		Ok(IActiveVibrationDeviceList(self.0.duplicate()?))
	}
}

impl<T> Deref for IActiveVibrationDeviceList<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IActiveVibrationDeviceList<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IActiveVibrationDeviceList<T> {
	pub fn activate_vibration_device(&self, unk0: ::nn::hid::VibrationDeviceHandle) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IActiveVibrationDeviceList<T> {
	fn from(obj: T) -> IActiveVibrationDeviceList<T> {
		IActiveVibrationDeviceList(obj)
	}
}
