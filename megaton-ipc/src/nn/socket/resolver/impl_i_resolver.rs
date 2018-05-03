
use megaton_hammer::kernel::{KObject, Session, Domain, Object};
use megaton_hammer::error::*;
use core::ops::{Deref, DerefMut};
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IResolver<T>(T);

impl IResolver<Session> {
	pub fn new() -> Result<Arc<IResolver<Session>>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IResolver<Session>>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"sfdnsres") {
			let ret = Arc::new(IResolver(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"sfdnsres").map(|s: KObject| Arc::new(Session::from(s).into()));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}

	pub fn to_domain(self) -> ::core::result::Result<IResolver<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IResolver(domain)),
			Err((sess, err)) => Err((IResolver(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IResolver<Session>> {
		Ok(IResolver(self.0.duplicate()?))
	}
}

impl<T> Deref for IResolver<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IResolver<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IResolver<T> {
	// fn unknown0(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unknown1(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_by_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_by_addr(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_host_string_error(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_gai_string_error(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_addr_info(&self, unk0: u8, unk1: u32, unk2: u64, unk4: &[u8], unk5: &[u8], unk6: &[u8; 0x400], unk10: &mut [u8; 0x1000]) -> Result<(u32, u32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u32,
			unk2: u64,
		}
		let req = Request::new(6)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			.send_pid()
			.descriptor(IPCBuffer::from_slice(unk4, 5))
			.descriptor(IPCBuffer::from_slice(unk5, 5))
			.descriptor(IPCBuffer::from_ref(unk6, 5))
			.descriptor(IPCBuffer::from_mut_ref(unk10, 6))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk7: u32,
			unk8: u32,
			unk9: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk7.clone(),res.get_raw().unk8.clone(),res.get_raw().unk9.clone()))
	}

	// fn get_name_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn request_cancel_handle(&self, unk0: u64) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(8)
			.args(unk0)
			.send_pid()
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn cancel_socket_call(&self, unk0: u32, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(9)
			.args(InRaw {
				unk0,
				unk1,
			})
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn unknown10(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(10)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn clear_dns_ip_server_address_array(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IResolver<T> {
	fn from(obj: T) -> IResolver<T> {
		IResolver(obj)
	}
}
