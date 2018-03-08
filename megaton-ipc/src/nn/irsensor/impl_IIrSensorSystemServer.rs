
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

pub struct IIrSensorSystemServer(Session);

impl IIrSensorSystemServer {
	pub fn SetAppletResourceUserId(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(500)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn RegisterAppletResourceUserId(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(501)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn UnregisterAppletResourceUserId(&self, unk0: ::nn::applet::AppletResourceUserId) -> Result<()> {
		let req = Request::new(502)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn EnableAppletToGetInput(&self, unk0: bool, unk1: ::nn::applet::AppletResourceUserId) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::nn::applet::AppletResourceUserId,
		}
		let req = Request::new(503)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IIrSensorSystemServer {
	unsafe fn from_kobject(obj: KObject) -> IIrSensorSystemServer {
		IIrSensorSystemServer(Session::from_kobject(obj))
	}
}
