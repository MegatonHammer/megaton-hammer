
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IGrcService(Session);

impl IGrcService {
	pub fn new() -> Result<IGrcService> {
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;
		let r = sm.get_service(*b"grc:c\0\0\0").map(|s| unsafe { IGrcService::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IGrcService {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IGrcService {
	pub fn get_continuous_recorder(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_game_movie_trimmer(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IGrcService {
	unsafe fn from_kobject(obj: KObject) -> IGrcService {
		IGrcService(Session::from_kobject(obj))
	}
}
