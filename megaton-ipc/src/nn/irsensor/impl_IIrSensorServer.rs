
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IIrSensorServer(Session);

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
	// fn GetIrsensorSharedMemoryHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
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
	// fn RunImageTransferProcessor(&self, UNKNOWN) -> Result<UNKNOWN>;
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
