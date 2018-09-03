
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct INetworkInstallManager<T>(T);

impl INetworkInstallManager<Session> {
	pub fn raw_new() -> Result<INetworkInstallManager<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"nim\0\0\0\0\0")?;
		let object : Self = Session::from(session).into();
		Ok(object)
	}

	pub fn new() -> Result<Arc<INetworkInstallManager<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<INetworkInstallManager<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"nim\0\0\0\0\0") {
			let ret = Arc::new(INetworkInstallManager(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = Self::raw_new()?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<INetworkInstallManager<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(INetworkInstallManager(domain)),
			Err((sess, err)) => Err((INetworkInstallManager(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<INetworkInstallManager<Session>> {
		Ok(INetworkInstallManager(self.0.duplicate()?))
	}
}

impl<T> Deref for INetworkInstallManager<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for INetworkInstallManager<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> INetworkInstallManager<T> {
	// fn create_system_update_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn destroy_system_update_task(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_system_update_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_system_update_task_run(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_system_update_task_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn commit_system_update_task(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn create_network_install_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn destroy_network_install_task(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_network_install_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_network_install_task_run(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_network_install_task_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn commit_network_install_task(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn request_latest_system_update_meta(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_application_network_install_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_network_install_task_content_meta(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_latest_version(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_network_install_task_attribute(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn add_network_install_task_content_meta(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_downloaded_system_data_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn calculate_network_install_task_required_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn is_ex_fat_driver_included(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_background_download_stress_task_info(&self, ) -> Result<u128> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn request_device_authentication_token(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_game_card_registration_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_register_game_card(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_register_notification_token(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_download_task_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_application_control(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_latest_application_control(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_version_list(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn create_apply_delta_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn destroy_apply_delta_task(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(32)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_application_apply_delta_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_apply_delta_task_run(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_apply_delta_task_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_apply_delta_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn commit_apply_delta_task(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(37)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn calculate_apply_delta_task_required_size(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn prepare_shutdown(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(39)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn list_apply_delta_task2(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn clear_not_enough_space_state_of_apply_delta_task(&self, unk0: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(41)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown42(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn unknown43(&self, ) -> Result<u128> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(43)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn unknown44(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown45(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn unknown46(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(46)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for INetworkInstallManager<T> {
	fn from(obj: T) -> INetworkInstallManager<T> {
		INetworkInstallManager(obj)
	}
}
