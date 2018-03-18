
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IIrSensorServer(Session);

impl IIrSensorServer {
	pub fn new() -> Result<Arc<IIrSensorServer>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IIrSensorServer>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"irs\0\0\0\0\0").map(|s| Arc::new(unsafe { IIrSensorServer::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IIrSensorServer {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IIrSensorServer {
	pub fn activate_irsensor(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(302)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn deactivate_irsensor(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(303)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_irsensor_shared_memory_handle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(304)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn stop_image_processor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::irsensor::IrCameraHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(305)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn run_moment_processor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId, unk2: ::nn::irsensor::PackedMomentProcessorConfig) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::irsensor::IrCameraHandle,
			unk1: ::nn::applet::AppletResourceUserId,
			unk2: ::nn::irsensor::PackedMomentProcessorConfig,
		}
		let req = Request::new(306)
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

	pub fn run_clustering_processor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId, unk2: ::nn::irsensor::PackedClusteringProcessorConfig) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::irsensor::IrCameraHandle,
			unk1: ::nn::applet::AppletResourceUserId,
			unk2: ::nn::irsensor::PackedClusteringProcessorConfig,
		}
		let req = Request::new(307)
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

	pub fn run_image_transfer_processor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId, unk2: ::nn::irsensor::PackedImageTransferProcessorConfig, unk3: u64, unk5: &KObject) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::irsensor::IrCameraHandle,
			unk1: ::nn::applet::AppletResourceUserId,
			unk2: ::nn::irsensor::PackedImageTransferProcessorConfig,
			unk3: u64,
		}
		let req = Request::new(308)
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
	pub fn get_npad_ir_camera_handle(&self, unk0: u32) -> Result<::nn::irsensor::IrCameraHandle> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(311)
			.args(unk0)
			;
		let res : Response<::nn::irsensor::IrCameraHandle> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn run_dpd_processor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn suspend_image_processor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::irsensor::IrCameraHandle,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(313)
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
	pub fn check_firmware_version(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::irsensor::PackedMcuVersion, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::nn::irsensor::IrCameraHandle,
			unk1: ::nn::irsensor::PackedMcuVersion,
			unk2: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(314)
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

impl FromKObject for IIrSensorServer {
	unsafe fn from_kobject(obj: KObject) -> IIrSensorServer {
		IIrSensorServer(Session::from_kobject(obj))
	}
}
