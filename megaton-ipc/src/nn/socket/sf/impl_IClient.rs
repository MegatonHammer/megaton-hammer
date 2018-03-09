
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};

#[derive(Debug)]
pub struct IClient(Session);

impl IClient {
	pub fn get_service() -> Result<IClient> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"bsd:u\0\0\0").map(|s| unsafe { IClient::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		let r = sm.GetService(*b"bsd:s\0\0\0").map(|s| unsafe { IClient::from_kobject(s) });
		if let Ok(service) = r {
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IClient {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IClient {
	pub fn Initialize(&self, config: ::nn::socket::BsdBufferConfig, pid: u64, transferMemorySize: u64, unk3: &KObject) -> Result<u32> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			config: ::nn::socket::BsdBufferConfig,
			pid: u64,
			transferMemorySize: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				config,
				pid,
				transferMemorySize,
			})
			.send_pid()
			.copy_handle(unk3)
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn StartMonitoring(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn Socket(&self, domain: u32, ty: u32, protocol: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			domain: u32,
			ty: u32,
			protocol: u32,
		}
		let req = Request::new(2)
			.args(InRaw {
				domain,
				ty,
				protocol,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn SocketExempt(&self, unk0: u32, unk1: u32, unk2: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
		}
		let req = Request::new(3)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn Open(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Select(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Poll(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Sysctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Recv(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RecvFrom(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Send(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SendTo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Accept(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Bind(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Connect(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetPeerName(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetSockName(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetSockOpt(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Listen(&self, socket: u32, backlog: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			backlog: u32,
		}
		let req = Request::new(18)
			.args(InRaw {
				socket,
				backlog,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn Ioctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Fcntl(&self, unk0: u32, unk1: u32, unk2: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
			unk2: u32,
		}
		let req = Request::new(20)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn SetSockOpt(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Shutdown(&self, socket: u32, how: u32) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			how: u32,
		}
		let req = Request::new(22)
			.args(InRaw {
				socket,
				how,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn ShutdownAllSockets(&self, how: u32) -> Result<(i32, u32)> {
		let req = Request::new(23)
			.args(how)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn Write(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn Read(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Close(&self, socket: u32) -> Result<(i32, u32)> {
		let req = Request::new(26)
			.args(socket)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn DuplicateSocket(&self, unk0: u32, unk1: u64) -> Result<(i32, u32)> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u64,
		}
		let req = Request::new(27)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let mut res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn GetResourceStatistics(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn RecvMMsg(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SendMMsg(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IClient {
	unsafe fn from_kobject(obj: KObject) -> IClient {
		IClient(Session::from_kobject(obj))
	}
}
