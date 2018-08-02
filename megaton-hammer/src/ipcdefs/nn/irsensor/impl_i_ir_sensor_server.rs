
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IIrSensorServer<T>(T);

impl IIrSensorServer<Session> {
	pub fn raw_new() -> Result<IIrSensorServer<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"irs\0\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<IIrSensorServer<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IIrSensorServer<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"irs\0\0\0\0\0") {
			let ret = Arc::new(IIrSensorServer(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IIrSensorServer<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IIrSensorServer(domain)),
			Err((sess, err)) => Err((IIrSensorServer(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IIrSensorServer<Session>> {
		Ok(IIrSensorServer(self.0.duplicate()?))
	}
}

impl<T> Deref for IIrSensorServer<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IIrSensorServer<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IIrSensorServer<T> {
	pub fn activate_irsensor(&self, unk0: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(302)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_irsensor(&self, unk0: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(303)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_irsensor_shared_memory_handle(&self, unk0: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(304)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn stop_image_processor(&self, unk0: ::ipcdefs::nn::irsensor::IrCameraHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::irsensor::IrCameraHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(305)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn run_moment_processor(&self, unk0: ::ipcdefs::nn::irsensor::IrCameraHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId, unk2: ::ipcdefs::nn::irsensor::PackedMomentProcessorConfig) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::irsensor::IrCameraHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
			unk2: ::ipcdefs::nn::irsensor::PackedMomentProcessorConfig,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(306)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn run_clustering_processor(&self, unk0: ::ipcdefs::nn::irsensor::IrCameraHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId, unk2: ::ipcdefs::nn::irsensor::PackedClusteringProcessorConfig) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::irsensor::IrCameraHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
			unk2: ::ipcdefs::nn::irsensor::PackedClusteringProcessorConfig,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(307)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn run_image_transfer_processor(&self, unk0: ::ipcdefs::nn::irsensor::IrCameraHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId, unk2: ::ipcdefs::nn::irsensor::PackedImageTransferProcessorConfig, unk3: u64, unk5: &KObject) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::irsensor::IrCameraHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
			unk2: ::ipcdefs::nn::irsensor::PackedImageTransferProcessorConfig,
			unk3: u64,
		}
		let req : Request<_, [_; 0], [_; 1], [_; 0]> = Request::new(308)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			.send_pid()
			.copy_handle(unk5)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_image_transfer_processor_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn run_tera_plugin_processor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_npad_ir_camera_handle(&self, unk0: u32) -> Result<::ipcdefs::nn::irsensor::IrCameraHandle> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(311)
			.args(unk0)
			;
		let res : Response<::ipcdefs::nn::irsensor::IrCameraHandle> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn run_dpd_processor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn suspend_image_processor(&self, unk0: ::ipcdefs::nn::irsensor::IrCameraHandle, unk1: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::irsensor::IrCameraHandle,
			unk1: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(313)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn check_firmware_version(&self, unk0: ::ipcdefs::nn::irsensor::IrCameraHandle, unk1: ::ipcdefs::nn::irsensor::PackedMcuVersion, unk2: ::ipcdefs::nn::applet::AppletResourceUserId) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::irsensor::IrCameraHandle,
			unk1: ::ipcdefs::nn::irsensor::PackedMcuVersion,
			unk2: ::ipcdefs::nn::applet::AppletResourceUserId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(314)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IIrSensorServer<T> {
	fn from(obj: T) -> IIrSensorServer<T> {
		IIrSensorServer(obj)
	}
}
