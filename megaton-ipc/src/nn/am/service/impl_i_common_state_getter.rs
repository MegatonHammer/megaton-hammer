
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ICommonStateGetter<T>(T);

impl ICommonStateGetter<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ICommonStateGetter<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ICommonStateGetter(domain)),
			Err((sess, err)) => Err((ICommonStateGetter(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ICommonStateGetter<Session>> {
		Ok(ICommonStateGetter(self.0.duplicate()?))
	}
}

impl<T> Deref for ICommonStateGetter<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ICommonStateGetter<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ICommonStateGetter<T> {
	pub fn get_event_handle(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn receive_message(&self, ) -> Result<::nn::am::AppletMessage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let res : Response<::nn::am::AppletMessage> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_this_applet_kind(&self, ) -> Result<::nn::am::service::AppletKind> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let res : Response<::nn::am::service::AppletKind> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn allow_to_enter_sleep(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disallow_to_enter_sleep(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_operation_mode(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_performance_mode(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_cradle_status(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_boot_mode(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_current_focus_state(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn request_to_acquire_sleep_lock(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn release_sleep_lock(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn release_sleep_lock_transiently(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_acquired_sleep_lock_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn push_to_general_channel(&self, unk0: &::nn::am::service::IStorage<Session>) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			.copy_handle(unk0.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_home_button_reader_lock_accessor(&self, ) -> Result<::nn::am::service::ILockAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(30)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_reader_lock_accessor_ex(&self, unk0: i32) -> Result<::nn::am::service::ILockAccessor<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_cradle_fw_version(&self, ) -> Result<(u32, u32, u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(40)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
			unk3: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone(),res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn is_vr_mode_enabled(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(50)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_vr_mode_enabled(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(51)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_in_controller_firmware_update_section(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(55)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_default_display_resolution(&self, ) -> Result<(i32, i32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(60)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: i32,
			unk1: i32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn get_default_display_resolution_change_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(61)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for ICommonStateGetter<T> {
	fn from(obj: T) -> ICommonStateGetter<T> {
		ICommonStateGetter(obj)
	}
}
