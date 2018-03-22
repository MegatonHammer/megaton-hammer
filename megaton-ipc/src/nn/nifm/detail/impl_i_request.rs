
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;

#[derive(Debug)]
pub struct IRequest(Session);

impl AsRef<Session> for IRequest {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IRequest {
	pub fn get_request_state(&self, ) -> Result<i32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(0)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_result(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_system_event_readable_handles(&self, ) -> Result<(KObject, KObject)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok((res.pop_handle(),res.pop_handle()))
	}

	pub fn cancel(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn submit(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_requirement(&self, unk0: ::nn::nifm::Requirement) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(5)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_requirement_preset(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(6)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_priority(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_network_profile_id(&self, unk0: ::nn::util::Uuid) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_rejectable(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_connection_confirmation_option(&self, unk0: i8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_persistent(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_instant(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_sustainable(&self, unk0: bool, unk1: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: u8,
		}
		let req = Request::new(14)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_raw_priority(&self, unk0: u8) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_greedy(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_sharable(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(17)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_requirement_by_revision(&self, unk0: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(18)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_requirement(&self, ) -> Result<::nn::nifm::Requirement> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(())
			;
		let res : Response<::nn::nifm::Requirement> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_revision(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(20)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_applet_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_additional_info(&self, unk1: &mut ::nn::nifm::AdditionalInfo) -> Result<u32> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(22)
			.args(())
			.descriptor(IPCBuffer::from_mut_ref(unk1, 0x16))
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_kept_in_sleep(&self, unk0: bool) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn register_socket_descriptor(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(24)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unregister_socket_descriptor(&self, unk0: i32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl FromKObject for IRequest {
	unsafe fn from_kobject(obj: KObject) -> IRequest {
		IRequest(Session::from_kobject(obj))
	}
}
