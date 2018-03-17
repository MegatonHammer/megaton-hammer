
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IGeneralInterface(Session);

impl IGeneralInterface {
	pub fn new() -> Result<IGeneralInterface> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"spl:\0\0\0\0").map(|s| unsafe { IGeneralInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IGeneralInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IGeneralInterface {
	pub fn get_config(&self, unk0: u32) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(unk0)
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn user_exp_mod(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn generate_aes_kek(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn load_aes_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn generate_aes_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_config(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_random_bytes(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(7)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn load_secure_exp_mod_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn secure_exp_mod(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn is_development(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn generate_specific_aes_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn decrypt_privk(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn decrypt_aes_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn decrypt_aes_ctr(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn compute_cmac(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn load_rsa_oaep_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unwrap_rsa_oaep_wrapped_title_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(18)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn load_title_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn unwrap_aes_wrapped_title_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn lock_aes_engine(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn unlock_aes_engine(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_spl_wait_event(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn set_shared_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(24)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_shared_data(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IGeneralInterface {
	unsafe fn from_kobject(obj: KObject) -> IGeneralInterface {
		IGeneralInterface(Session::from_kobject(obj))
	}
}
