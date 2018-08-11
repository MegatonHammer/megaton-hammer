
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISession<T>(T);

impl ISession<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISession<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISession(domain)),
			Err((sess, err)) => Err((ISession(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISession<Session>> {
		Ok(ISession(self.0.duplicate()?))
	}
}

impl<T> Deref for ISession<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISession<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISession<T> {
	// fn query_functions(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn acquire_function(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn release_function(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_function_state(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_bar_profile(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_config(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_config(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_bar_region(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_bar_region(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn find_capability(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn find_extended_capability(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn map_dma(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unmap_dma(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn unmap_dma_bus_address(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_dma_bus_address(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_dma_bus_address_range(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_dma_enable(&self, unk0: u8, unk1: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn acquire_irq(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn release_irq(&self, unk0: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(18)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_irq_enable(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_aspm_enable(&self, unk0: u8, unk1: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for ISession<T> {
	fn from(obj: T) -> ISession<T> {
		ISession(obj)
	}
}
