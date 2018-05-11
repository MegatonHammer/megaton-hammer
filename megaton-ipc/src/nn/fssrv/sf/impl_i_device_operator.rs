
use megaton_hammer::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use megaton_hammer::kernel::KObject;
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDeviceOperator<T>(T);

impl IDeviceOperator<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDeviceOperator<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDeviceOperator(domain)),
			Err((sess, err)) => Err((IDeviceOperator(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDeviceOperator<Session>> {
		Ok(IDeviceOperator(self.0.duplicate()?))
	}
}

impl<T> Deref for IDeviceOperator<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDeviceOperator<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDeviceOperator<T> {
	pub fn is_sd_card_inserted(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_sd_card_speed_mode(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_sd_card_cid(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn get_sd_card_user_area_size(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_sd_card_protected_area_size(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_and_clear_sd_card_error_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_mmc_cid(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_mmc_speed_mode(&self, ) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(101)
			.args(())
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn erase_mmc(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(110)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_mmc_partition_size(&self, unk0: u32) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(111)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_mmc_patrol_count(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(112)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_and_clear_mmc_error_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_mmc_extended_csd(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn is_game_card_inserted(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(200)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn erase_game_card(&self, unk0: u32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(201)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_game_card_handle(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(202)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_game_card_update_partition_info(&self, unk0: u32) -> Result<(u32, ::nn::ApplicationId)> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(203)
			.args(unk0)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			version: u32,
			tid: ::nn::ApplicationId,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().version.clone(),res.get_raw().tid.clone()))
	}

	pub fn finalize_game_card_driver(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(204)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_game_card_attribute(&self, unk0: u32) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(205)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_game_card_device_certificate(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_game_card_asic_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_game_card_id_set(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_to_game_card(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_verify_write_enalble_flag(&self, flag: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(210)
			.args(flag)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_game_card_image_hash(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_game_card_device_id_for_prod_card(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn erase_and_write_param_directly(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_game_card_cid(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn force_erase_game_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(215)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_game_card_error_info(&self, ) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(216)
			.args(())
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_game_card_error_report_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_game_card_device_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_speed_emulation_mode(&self, mode: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(300)
			.args(mode)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_speed_emulation_mode(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(301)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl<T: Object> From<T> for IDeviceOperator<T> {
	fn from(obj: T) -> IDeviceOperator<T> {
		IDeviceOperator(obj)
	}
}
