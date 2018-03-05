
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::ll::{Request, Response};

pub struct ISharedFontManager(Session);

impl ISharedFontManager {
	pub fn RequestLoad(&self, unk0: u32) -> Result<()> {
		let req = Request::new(0)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}
	pub fn GetLoadState(&self, unk0: u32) -> Result<u32> {
		let req = Request::new(1)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetSize(&self, unk0: u32) -> Result<u32> {
		let req = Request::new(2)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	pub fn GetSharedMemoryAddressOffset(&self, unk0: u32) -> Result<u32> {
		let req = Request::new(3)
			.args(unk0)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}
	// fn GetSharedMemoryNativeHandle(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetSharedFontInOrderOfPriority(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for ISharedFontManager {
	unsafe fn from_kobject(obj: KObject) -> ISharedFontManager {
		ISharedFontManager(Session::from_kobject(obj))
	}
}
