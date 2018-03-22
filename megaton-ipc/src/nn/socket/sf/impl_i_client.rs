
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IClient(Session);

impl IClient {
	pub fn new_bsd_u() -> Result<Arc<IClient>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IClient>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"bsd:u\0\0\0") {
			let ret = Arc::new(IClient(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"bsd:u\0\0\0").map(|s| Arc::new(unsafe { IClient::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
	pub fn new_bsd_s() -> Result<Arc<IClient>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IClient>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		if let Some(hnd) = ::megaton_hammer::loader::get_override_service(*b"bsd:s\0\0\0") {
			let ret = Arc::new(IClient(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let r = sm.get_service(*b"bsd:s\0\0\0").map(|s| Arc::new(unsafe { IClient::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
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
	pub fn initialize(&self, config: ::nn::socket::BsdBufferConfig, pid: u64, transfer_memory_size: u64, unk3: &KObject) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			config: ::nn::socket::BsdBufferConfig,
			pid: u64,
			transfer_memory_size: u64,
		}
		let req = Request::new(0)
			.args(InRaw {
				config,
				pid,
				transfer_memory_size,
			})
			.send_pid()
			.copy_handle(unk3)
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn start_monitoring(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn socket(&self, domain: u32, ty: u32, protocol: u32) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn socket_exempt(&self, unk0: u32, unk1: u32, unk2: u32) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn open(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn select(&self, nfds: u32, timeout: ::nn::socket::Timeout, readfds_in: &[::nn::socket::FdSet], writefds_in: &[::nn::socket::FdSet], errorfds_in: &[::nn::socket::FdSet], readfds_out: &mut [::nn::socket::FdSet], writefds_out: &mut [::nn::socket::FdSet], errorfds_out: &mut [::nn::socket::FdSet]) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			nfds: u32,
			timeout: ::nn::socket::Timeout,
		}
		let req = Request::new(5)
			.args(InRaw {
				nfds,
				timeout,
			})
			.descriptor(IPCBuffer::from_slice(readfds_in, 0x21))
			.descriptor(IPCBuffer::from_slice(writefds_in, 0x21))
			.descriptor(IPCBuffer::from_slice(errorfds_in, 0x21))
			.descriptor(IPCBuffer::from_mut_slice(readfds_out, 0x22))
			.descriptor(IPCBuffer::from_mut_slice(writefds_out, 0x22))
			.descriptor(IPCBuffer::from_mut_slice(errorfds_out, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn poll(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn sysctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn recv(&self, socket: u32, flags: u32, message: &mut [i8]) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			flags: u32,
		}
		let req = Request::new(8)
			.args(InRaw {
				socket,
				flags,
			})
			.descriptor(IPCBuffer::from_mut_slice(message, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn recv_from(&self, sock: u32, flags: u32, message: &mut [i8], unk6: &mut ::nn::socket::Sockaddr) -> Result<(i32, u32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			sock: u32,
			flags: u32,
		}
		let req = Request::new(9)
			.args(InRaw {
				sock,
				flags,
			})
			.descriptor(IPCBuffer::from_mut_slice(message, 0x22))
			.descriptor(IPCBuffer::from_mut_ref(unk6, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	pub fn send(&self, socket: u32, flags: u32, unk2: &[i8]) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			flags: u32,
		}
		let req = Request::new(10)
			.args(InRaw {
				socket,
				flags,
			})
			.descriptor(IPCBuffer::from_slice(unk2, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn send_to(&self, socket: u32, flags: u32, unk2: &[i8], unk3: &::nn::socket::Sockaddr) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			socket: u32,
			flags: u32,
		}
		let req = Request::new(11)
			.args(InRaw {
				socket,
				flags,
			})
			.descriptor(IPCBuffer::from_slice(unk2, 0x21))
			.descriptor(IPCBuffer::from_ref(unk3, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn accept(&self, socket: u32, addr: &mut ::nn::socket::Sockaddr) -> Result<(i32, u32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_ref(addr, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	pub fn bind(&self, socket: u32, unk1: &::nn::socket::Sockaddr) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(socket)
			.descriptor(IPCBuffer::from_ref(unk1, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn connect(&self, socket: u32, unk1: &::nn::socket::Sockaddr) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(14)
			.args(socket)
			.descriptor(IPCBuffer::from_ref(unk1, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn get_peer_name(&self, socket: u32, addr: &mut ::nn::socket::Sockaddr) -> Result<(i32, u32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(15)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_ref(addr, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	pub fn get_sock_name(&self, socket: u32, addr: &mut ::nn::socket::Sockaddr) -> Result<(i32, u32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(16)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_ref(addr, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
			addrlen: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone(),res.get_raw().addrlen.clone()))
	}

	// fn get_sock_opt(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn listen(&self, socket: u32, backlog: u32) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn ioctl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn fcntl(&self, unk0: u32, unk1: u32, unk2: u32) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn set_sock_opt(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn shutdown(&self, socket: u32, how: u32) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn shutdown_all_sockets(&self, how: u32) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(23)
			.args(how)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn write(&self, socket: u32, message: &[i8]) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(24)
			.args(socket)
			.descriptor(IPCBuffer::from_slice(message, 0x21))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn read(&self, socket: u32, message: &mut [i8]) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::IPCBuffer;
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(25)
			.args(socket)
			.descriptor(IPCBuffer::from_mut_slice(message, 0x22))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn close(&self, socket: u32) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(26)
			.args(socket)
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			ret: i32,
			bsd_errno: u32,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	pub fn duplicate_socket(&self, unk0: u32, unk1: u64) -> Result<(i32, u32)> {
		use megaton_hammer::ipc::{Request, Response};

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
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().ret.clone(),res.get_raw().bsd_errno.clone()))
	}

	// fn get_resource_statistics(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn recv_m_msg(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn send_m_msg(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IClient {
	unsafe fn from_kobject(obj: KObject) -> IClient {
		IClient(Session::from_kobject(obj))
	}
}
