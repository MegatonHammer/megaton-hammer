
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct IHOSBinderDriver(Session);

impl IHOSBinderDriver {
	// fn TransactParcel(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn AdjustRefcount(&self, unk0: i32, unk1: i32, unk2: i32) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: i32,
			unk1: i32,
			unk2: i32,
		}
		let req = Request::new(1)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	// fn GetNativeHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn TransactParcelAuto(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IHOSBinderDriver {
	unsafe fn from_kobject(obj: KObject) -> IHOSBinderDriver {
		IHOSBinderDriver(Session::from_kobject(obj))
	}
}
