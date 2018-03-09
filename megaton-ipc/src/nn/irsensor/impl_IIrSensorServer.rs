
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IIrSensorServer(Session);

impl IIrSensorServer {
	pub fn get_service() -> Result<IIrSensorServer> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"irs\0\0\0\0\0").map(|s| unsafe { IIrSensorServer::from_kobject(s) });
		if let Ok(service) = r {
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
	pub fn ActivateIrsensor(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(302)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeactivateIrsensor(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(303)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetIrsensorSharedMemoryHandle(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<KObject> {
		let req = Request::new(304)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn StopImageProcessor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RunMomentProcessor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId, unk2: ::nn::irsensor::PackedMomentProcessorConfig) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RunClusteringProcessor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId, unk2: ::nn::irsensor::PackedClusteringProcessorConfig) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RunImageTransferProcessor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId, unk2: ::nn::irsensor::PackedImageTransferProcessorConfig, unk3: u64, unk5: &KObject) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetImageTransferProcessorState(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RunTeraPluginProcessor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn GetNpadIrCameraHandle(&self, unk0: u32) -> Result<::nn::irsensor::IrCameraHandle> {
		let req = Request::new(311)
			.args(unk0)
			;
		let mut res : Response<::nn::irsensor::IrCameraHandle> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn RunDpdProcessor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SuspendImageProcessor(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn CheckFirmwareVersion(&self, unk0: ::nn::irsensor::IrCameraHandle, unk1: ::nn::irsensor::PackedMcuVersion, unk2: ::nn::applet::AppletResourceUserId) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IIrSensorServer {
	unsafe fn from_kobject(obj: KObject) -> IIrSensorServer {
		IIrSensorServer(Session::from_kobject(obj))
	}
}
