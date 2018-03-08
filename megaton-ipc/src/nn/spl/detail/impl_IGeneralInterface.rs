
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IGeneralInterface(Session);

impl IGeneralInterface {
	pub fn get_service() -> Result<IGeneralInterface> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"spl:\0\0\0\0").map(|s| unsafe { IGeneralInterface::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl IGeneralInterface {
	pub fn GetConfig(&self, unk0: u32) -> Result<u64> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn UserExpMod(&self, ) -> Result<()> {
		let req = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GenerateAesKek(&self, ) -> Result<()> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn LoadAesKey(&self, ) -> Result<()> {
		let req = Request::new(3)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GenerateAesKey(&self, ) -> Result<()> {
		let req = Request::new(4)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetConfig(&self, ) -> Result<()> {
		let req = Request::new(5)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetRandomBytes(&self, ) -> Result<()> {
		let req = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DecryptImportPrivkForRsaOaep(&self, ) -> Result<()> {
		let req = Request::new(9)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DecryptRsaOaep(&self, ) -> Result<()> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn IsDevelopment(&self, ) -> Result<u8> {
		let req = Request::new(11)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GenerateSpecificAesKey(&self, ) -> Result<()> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DecryptPrivk(&self, ) -> Result<()> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DecryptAesKey(&self, ) -> Result<()> {
		let req = Request::new(14)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DecryptAesCtr(&self, ) -> Result<()> {
		let req = Request::new(15)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ComputeCmac(&self, ) -> Result<()> {
		let req = Request::new(16)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DecryptImportPrivkForExpMod1(&self, ) -> Result<()> {
		let req = Request::new(17)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnwrapRsaWrappedTitleKey(&self, ) -> Result<()> {
		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn LoadTitleKey(&self, ) -> Result<()> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn UnwrapAesWrappedTitleKey(&self, ) -> Result<()> {
		let req = Request::new(20)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn LockAesEngine(&self, ) -> Result<u32> {
		let req = Request::new(21)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn UnlockAesEngine(&self, unk0: u32) -> Result<()> {
		let req = Request::new(22)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn GetSplWaitEvent(&self, ) -> Result<KObject> {
		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

}

impl FromKObject for IGeneralInterface {
	unsafe fn from_kobject(obj: KObject) -> IGeneralInterface {
		IGeneralInterface(Session::from_kobject(obj))
	}
}
