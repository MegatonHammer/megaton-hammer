
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IApplicationDisplayService<T>(T);

impl IApplicationDisplayService<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IApplicationDisplayService<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IApplicationDisplayService(domain)),
			Err((sess, err)) => Err((IApplicationDisplayService(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IApplicationDisplayService<Session>> {
		Ok(IApplicationDisplayService(self.0.duplicate()?))
	}
}

impl<T> Deref for IApplicationDisplayService<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IApplicationDisplayService<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IApplicationDisplayService<T> {
	pub fn get_relay_service(&self, ) -> Result<::nns::hosbinder::IHOSBinderDriver<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_system_display_service(&self, ) -> Result<::nn::visrv::sf::ISystemDisplayService<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_manager_display_service(&self, ) -> Result<::nn::visrv::sf::IManagerDisplayService<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_indirect_display_transaction_service(&self, ) -> Result<::nns::hosbinder::IHOSBinderDriver<T>> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(103)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn list_displays(&self, unk1: &mut [::nn::vi::DisplayInfo]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1000)
			.args(())
			.descriptor(IPCBuffer::from_mut_slice(unk1, 6))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn open_display(&self, unk0: ::nn::vi::DisplayName) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1010)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn open_default_display(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1011)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn close_display(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1020)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_display_enabled(&self, unk0: bool, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u64,
		}
		let req = Request::new(1101)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_display_resolution(&self, unk0: u64) -> Result<(i64, i64)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1102)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: i64,
			unk2: i64,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().unk2.clone()))
	}

	pub fn open_layer(&self, unk0: ::nn::vi::DisplayName, unk1: u64, unk2: ::nn::applet::AppletResourceUserId, unk5: &mut [u8; 0x100]) -> Result<i64> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::vi::DisplayName,
			unk1: u64,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(2020)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_mut_ref(unk5, 6))
			;
		let res : Response<i64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn close_layer(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2021)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_stray_layer(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn destroy_stray_layer(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2031)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_layer_scaling_mode(&self, unk0: u32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(2101)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_indirect_layer_image_map(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_indirect_layer_image_crop_map(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_indirect_layer_image_required_memory_info(&self, unk0: i64, unk1: i64) -> Result<(i64, i64)> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i64,
			unk1: i64,
		}
		let req = Request::new(2460)
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

	pub fn get_display_vsync_event(&self, unk0: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5202)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn get_display_vsync_event_for_debug(&self, unk0: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5203)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl<T: Object> From<T> for IApplicationDisplayService<T> {
	fn from(obj: T) -> IApplicationDisplayService<T> {
		IApplicationDisplayService(obj)
	}
}
