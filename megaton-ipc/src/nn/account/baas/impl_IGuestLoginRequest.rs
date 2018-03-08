
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IGuestLoginRequest(Session);

impl IGuestLoginRequest {
	pub fn GetSessionId(&self, ) -> Result<::nn::account::detail::Uuid> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetAccountId(&self, ) -> Result<::nn::account::NetworkServiceAccountId> {
		let req = Request::new(12)
			.args(())
			;
		let mut res : Response<::nn::account::NetworkServiceAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetLinkedNintendoAccountId(&self, ) -> Result<::nn::account::NintendoAccountId> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<::nn::account::NintendoAccountId> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn GetNickname(&self, unk0: &mut [i8]) -> Result<()> {
		let req = Request::new(14)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetProfileImage(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn LoadIdTokenCache(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IGuestLoginRequest {
	unsafe fn from_kobject(obj: KObject) -> IGuestLoginRequest {
		IGuestLoginRequest(Session::from_kobject(obj))
	}
}
