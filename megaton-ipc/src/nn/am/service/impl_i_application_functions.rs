
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IApplicationFunctions<T>(T);

impl IApplicationFunctions<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IApplicationFunctions<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IApplicationFunctions(domain)),
			Err((sess, err)) => Err((IApplicationFunctions(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IApplicationFunctions<Session>> {
		Ok(IApplicationFunctions(self.0.duplicate()?))
	}
}

impl<T> Deref for IApplicationFunctions<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IApplicationFunctions<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IApplicationFunctions<T> {
	pub fn pop_launch_parameter(&self, unk0: u32) -> Result<::nn::am::service::IStorage<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn create_application_and_push_and_request_to_start(&self, unk0: ::nn::ncm::ApplicationId, unk1: &::nn::am::service::IStorage<Session>) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(10)
			.args(unk0)
			.copy_handle(unk1.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_application_and_push_and_request_to_start_for_quest(&self, unk0: u32, unk1: u32, unk2: ::nn::ncm::ApplicationId, unk3: &::nn::am::service::IStorage<Session>) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: ::nn::ncm::ApplicationId,
		}
		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(11)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.copy_handle(unk3.as_ref())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ensure_save_data(&self, unk0: ::nn::account::Uid) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(unk0)
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_desired_language(&self, ) -> Result<::nn::settings::LanguageCode> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(21)
			.args(())
			;
		let res : Response<::nn::settings::LanguageCode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_terminate_result(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_display_version(&self, ) -> Result<::nn::oe::DisplayVersion> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(())
			;
		let res : Response<::nn::oe::DisplayVersion> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_launch_storage_info_for_debug(&self, ) -> Result<(::nn::ncm::StorageId, ::nn::ncm::StorageId)> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(24)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: ::nn::ncm::StorageId,
			unk1: ::nn::ncm::StorageId,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn extend_save_data(&self, unk0: u8, unk1: ::nn::account::Uid, unk2: i64, unk3: i64) -> Result<i64> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: ::nn::account::Uid,
			unk2: i64,
			unk3: i64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_save_data_size(&self, unk0: u8, unk1: ::nn::account::Uid) -> Result<(i64, i64)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: ::nn::account::Uid,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(26)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk2: i64,
			unk3: i64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk2.clone(),res.get_raw().unk3.clone()))
	}

	pub fn begin_blocking_home_button_short_and_long_pressed(&self, unk0: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn end_blocking_home_button_short_and_long_pressed(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(31)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn begin_blocking_home_button(&self, unk0: i64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(32)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn end_blocking_home_button(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(33)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn notify_running(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(40)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_pseudo_device_id(&self, ) -> Result<::nn::util::Uuid> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(50)
			.args(())
			;
		let res : Response<::nn::util::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_media_playback_state_for_application(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(60)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_game_play_recording_supported(&self, ) -> Result<bool> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(65)
			.args(())
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn initialize_game_play_recording(&self, unk0: u64, unk1: &KObject) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(66)
			.args(unk0)
			.copy_handle(unk1)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_game_play_recording_state(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(67)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_to_shutdown(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(70)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_to_reboot(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(71)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IApplicationFunctions<T> {
	fn from(obj: T) -> IApplicationFunctions<T> {
		IApplicationFunctions(obj)
	}
}
