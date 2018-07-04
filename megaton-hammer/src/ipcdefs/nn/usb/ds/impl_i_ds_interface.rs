
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct IDsInterface<T>(T);

impl IDsInterface<Session> {
	pub fn to_domain(self) -> ::core::result::Result<IDsInterface<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IDsInterface(domain)),
			Err((sess, err)) => Err((IDsInterface(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IDsInterface<Session>> {
		Ok(IDsInterface(self.0.duplicate()?))
	}
}

impl<T> Deref for IDsInterface<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IDsInterface<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IDsInterface<T> {
	pub fn get_ds_endpoint(&self, unk0: &[::ipcdefs::nn::usb::UsbEndpointDescriptor]) -> Result<::ipcdefs::nn::usb::ds::IDsEndpoint<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(())
			.descriptor(IPCBuffer::from_slice(unk0, 5))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn get_setup_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn unknown2(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn enable_interface(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_interface(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn ctrl_in_post_buffer_async(&self, size: u32, buffer: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(5)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn ctrl_out_post_buffer_async(&self, size: u32, buffer: u64) -> Result<u32> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			size: u32,
			buffer: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(InRaw {
				size,
				buffer,
			})
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_ctrl_in_completion_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_ctrl_in_report_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_ctrl_out_completion_event(&self, ) -> Result<KObject> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(res.pop_handle())
	}

	// fn get_ctrl_out_report_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn stall_ctrl(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(11)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IDsInterface<T> {
	fn from(obj: T) -> IDsInterface<T> {
		IDsInterface(obj)
	}
}
