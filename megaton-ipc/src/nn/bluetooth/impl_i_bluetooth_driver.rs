
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IBluetoothDriver(Session);

impl IBluetoothDriver {
	pub fn new() -> Result<Arc<IBluetoothDriver>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IBluetoothDriver>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"btdrv\0\0\0") {
			let ret = Arc::new(IBluetoothDriver(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"btdrv\0\0\0").map(|s| Arc::new(unsafe { IBluetoothDriver::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IBluetoothDriver {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IBluetoothDriver {
	pub fn unknown0(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn init(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn enable(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cleanup_and_shutdown(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_adapter_properties(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_adapter_property(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_adapter_property(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn start_discovery(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn cancel_discovery(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_bond(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn remove_bond(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn cancel_bond(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn pin_reply(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ssp_reply(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown15(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn init_interfaces(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_connect(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_disconnect(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_send_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_send_data2(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_set_report(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_get_report(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_wake_controller(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_add_paired_device(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_get_paired_device(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn hid_host_interface_cleanup_and_shutdown(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(26)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown27(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ext_interface_set_tsi(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ext_interface_set_burst_mode(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ext_interface_set_zero_retran(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ext_interface_set_mc_mode(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(31)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ext_interface_start_llr_mode(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(32)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ext_interface_exit_llr_mode(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(33)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ext_interface_set_radio(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(34)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn ext_interface_set_visibility(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown36(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(36)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown37(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn hid_host_interface_get_latest_plr(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn ext_interface_get_pending_connections(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(39)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn hid_host_interface_get_channel_map(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_is_bluetooth_boost_enabled(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(41)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_is_bluetooth_boost_enabled(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(42)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_is_bluetooth_afh_enabled(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(43)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_is_bluetooth_afh_enabled(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(44)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IBluetoothDriver {
	unsafe fn from_kobject(obj: KObject) -> IBluetoothDriver {
		IBluetoothDriver(Session::from_kobject(obj))
	}
}
