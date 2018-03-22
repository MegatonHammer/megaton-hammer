
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IDownloadTaskInterface(Session);

impl AsRef<Session> for IDownloadTaskInterface {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IDownloadTaskInterface {
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

impl FromKObject for IDownloadTaskInterface {
	unsafe fn from_kobject(obj: KObject) -> IDownloadTaskInterface {
		IDownloadTaskInterface(Session::from_kobject(obj))
	}
}
