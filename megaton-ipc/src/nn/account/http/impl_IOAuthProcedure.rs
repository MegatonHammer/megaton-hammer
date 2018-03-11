
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hammer::ipc::IPCBuffer;

#[derive(Debug)]
pub struct IOAuthProcedure(Session);

impl AsRef<Session> for IOAuthProcedure {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IOAuthProcedure {
	pub fn PrepareAsync(&self, ) -> Result<::nn::account::detail::IAsyncContext> {
		let req = Request::new(0)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn GetRequest(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ApplyResponse(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ApplyResponseAsync(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Suspend(&self, ) -> Result<::nn::account::detail::Uuid> {
		let req = Request::new(10)
			.args(())
			;
		let mut res : Response<::nn::account::detail::Uuid> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for IOAuthProcedure {
	unsafe fn from_kobject(obj: KObject) -> IOAuthProcedure {
		IOAuthProcedure(Session::from_kobject(obj))
	}
}
