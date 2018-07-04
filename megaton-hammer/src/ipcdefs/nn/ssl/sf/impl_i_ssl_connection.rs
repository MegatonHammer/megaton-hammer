
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct ISslConnection<T>(T);

impl ISslConnection<Session> {
	pub fn to_domain(self) -> ::core::result::Result<ISslConnection<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(ISslConnection(domain)),
			Err((sess, err)) => Err((ISslConnection(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<ISslConnection<Session>> {
		Ok(ISslConnection(self.0.duplicate()?))
	}
}

impl<T> Deref for ISslConnection<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for ISslConnection<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> ISslConnection<T> {
	pub fn set_socket_descriptor(&self, unk0: i32) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(0)
			.args(unk0)
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn set_host_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_verify_option(&self, unk0: ::ipcdefs::nn::ssl::sf::VerifyOption) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_io_mode(&self, unk0: ::ipcdefs::nn::ssl::sf::IoMode) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(3)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_socket_descriptor(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(4)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_host_name(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn get_verify_option(&self, ) -> Result<::ipcdefs::nn::ssl::sf::VerifyOption> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(6)
			.args(())
			;
		let res : Response<::ipcdefs::nn::ssl::sf::VerifyOption> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_io_mode(&self, ) -> Result<::ipcdefs::nn::ssl::sf::IoMode> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(())
			;
		let res : Response<::ipcdefs::nn::ssl::sf::IoMode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn do_handshake(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(8)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn do_handshake_get_server_cert(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn pending(&self, ) -> Result<i32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(())
			;
		let res : Response<i32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn peek(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn poll(&self, unk0: ::ipcdefs::nn::ssl::sf::PollEvent, unk1: u32) -> Result<::ipcdefs::nn::ssl::sf::PollEvent> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: ::ipcdefs::nn::ssl::sf::PollEvent,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(14)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<::ipcdefs::nn::ssl::sf::PollEvent> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn get_verify_cert_error(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(15)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_needed_server_cert_buffer_size(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(16)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_session_cache_mode(&self, unk0: ::ipcdefs::nn::ssl::sf::SessionCacheMode) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(17)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_session_cache_mode(&self, ) -> Result<::ipcdefs::nn::ssl::sf::SessionCacheMode> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(18)
			.args(())
			;
		let res : Response<::ipcdefs::nn::ssl::sf::SessionCacheMode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn flush_session_cache(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(19)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_renegotiation_mode(&self, unk0: ::ipcdefs::nn::ssl::sf::RenegotiationMode) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(20)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_renegotiation_mode(&self, ) -> Result<::ipcdefs::nn::ssl::sf::RenegotiationMode> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(21)
			.args(())
			;
		let res : Response<::ipcdefs::nn::ssl::sf::RenegotiationMode> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn set_option(&self, unk0: bool, unk1: ::ipcdefs::nn::ssl::sf::OptionType) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: bool,
			unk1: ::ipcdefs::nn::ssl::sf::OptionType,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_option(&self, unk0: ::ipcdefs::nn::ssl::sf::OptionType) -> Result<bool> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(unk0)
			;
		let res : Response<bool> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn get_verify_cert_errors(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl<T: Object> From<T> for ISslConnection<T> {
	fn from(obj: T) -> ISslConnection<T> {
		ISslConnection(obj)
	}
}
