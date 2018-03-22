
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct ISystem(Session);

impl AsRef<Session> for ISystem {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl ISystem {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown1(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn unknown3(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown4(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown5(&self, unk0: u64, unk1: u32, unk2: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u32,
			unk2: u32,
		}
		let req = Request::new(5)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown6(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown10(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown11(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown13(&self, unk0: u64, unk1: &mut [u8; 0x58]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown14(&self, unk0: u64, unk1: &mut [u8; 0x100]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown15(&self, unk0: u64, unk1: &mut [u8; 0x40]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown16(&self, unk0: u64, unk1: &mut [u8; 0x40]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown17(&self, unk0: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown18(&self, unk0: u64) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(18)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown19(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown20(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown21(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(unk0)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn unknown23(&self, ) -> Result<KObject> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	pub fn unknown100(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown101(&self, unk0: u64, unk1: &mut [u8; 0x40]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(101)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown102(&self, unk0: u64, unk1: &mut [u8; 0x100]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(102)
			.args(unk0)
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x1a))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown103(&self, unk0: u64, unk1: &[u8; 0x100]) -> Result<()> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(103)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(unk1, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown104(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(104)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown105(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(105)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown106(&self, unk0: u64) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(106)
			.args(unk0)
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

}

impl FromKObject for ISystem {
	unsafe fn from_kobject(obj: KObject) -> ISystem {
		ISystem(Session::from_kobject(obj))
	}
}
