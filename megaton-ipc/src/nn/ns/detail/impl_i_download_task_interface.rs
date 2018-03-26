
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDownloadTaskInterface<T>(T);

impl IDownloadTaskInterface<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDownloadTaskInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDownloadTaskInterface(domain)),
			Err((sess, err)) => Err((IDownloadTaskInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDownloadTaskInterface<Session>> {
		Ok(IDownloadTaskInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IDownloadTaskInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDownloadTaskInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDownloadTaskInterface<T> {
	pub fn clear_task_status_list(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(701)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn request_download_task_list(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(702)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn request_ensure_download_task(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn list_download_task_status(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn request_download_task_list_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn try_commit_current_application_download_task(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(706)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn enable_auto_commit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(707)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_auto_commit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(708)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn trigger_dynamic_commit_event(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(709)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDownloadTaskInterface<T> {
	fn from(obj: T) -> IDownloadTaskInterface<T> {
		IDownloadTaskInterface(obj)
	}
}
