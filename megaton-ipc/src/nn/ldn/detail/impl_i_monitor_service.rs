
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IMonitorService(Session);

impl AsRef<Session> for IMonitorService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IMonitorService {
	pub fn get_state_for_monitor(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_network_info_for_monitor(&self, unk0: &mut [u8; 0x480]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk0, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_ipv4_address_for_monitor(&self, ) -> Result<(u32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk0: u32,
			unk1: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk0.clone(),res.get_raw().unk1.clone()))
	}

	pub fn get_disconnect_reason_for_monitor(&self, ) -> Result<u16> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let res : Response<u16> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_security_parameter_for_monitor(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_network_config_for_monitor(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn initialize_monitor(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn finalize_monitor(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IMonitorService {
	unsafe fn from_kobject(obj: KObject) -> IMonitorService {
		IMonitorService(Session::from_kobject(obj))
	}
}
