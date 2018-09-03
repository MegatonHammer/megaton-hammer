
use ::kernel::{Session, Domain, Object};
#[allow(unused_imports)]
use ::kernel::KObject;
use ::error::*;
use core::ops::{Deref, DerefMut};
use alloc::sync::Arc;

#[derive(Debug)]
pub struct IFileSystemProxy<T>(T);

impl IFileSystemProxy<Session> {
	pub fn raw_new(unk0: u64) -> Result<IFileSystemProxy<Session>> {
		use ::ipcdefs::nn::sm::detail::IUserInterface;

		let sm = IUserInterface::raw_new()?;

		let session = sm.get_service(*b"fsp-srv\0")?;
		let object : Self = Session::from(session).into();
		object.initialize(unk0)?;
		Ok(object)
	}

	pub fn new<T: FnOnce(fn(u64) -> Result<IFileSystemProxy<Session>>) -> Result<IFileSystemProxy<Session>>>(f: T) -> Result<Arc<IFileSystemProxy<Session>>> {
		use alloc::sync::Weak;
		use kernel::sync::InternalMutex;
		use core::mem::ManuallyDrop;
		lazy_static! {
			static ref HANDLE : InternalMutex<Weak<IFileSystemProxy<Session>>> = InternalMutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}

		if let Some(hnd) = ::loader::get_override_service(*b"fsp-srv\0") {
			let ret = Arc::new(IFileSystemProxy(ManuallyDrop::into_inner(hnd)));
			::core::mem::forget(ret.clone());
			*HANDLE.lock() = Arc::downgrade(&ret);
			return Ok(ret);
		}

		let hnd = f(Self::raw_new)?;
		let ret = Arc::new(hnd);
		*HANDLE.lock() = Arc::downgrade(&ret);
		Ok(ret)
	}

	pub fn to_domain(self) -> ::core::result::Result<IFileSystemProxy<Domain>, (Self, Error)> {
		match self.0.to_domain() {
			Ok(domain) => Ok(IFileSystemProxy(domain)),
			Err((sess, err)) => Err((IFileSystemProxy(sess), err))
		}
	}

	pub fn duplicate(&self) -> Result<IFileSystemProxy<Session>> {
		Ok(IFileSystemProxy(self.0.duplicate()?))
	}
}

impl<T> Deref for IFileSystemProxy<T> {
	type Target = T;
	fn deref(&self) -> &T {
		&self.0
	}
}
impl<T> DerefMut for IFileSystemProxy<T> {
	fn deref_mut(&mut self) -> &mut T {
		&mut self.0
	}
}
impl<T: Object> IFileSystemProxy<T> {
	#[cfg(not(feature = "switch-2.0.0"))]
	pub fn open_file_system(&self, filesystem_type: ::ipcdefs::nn::fssrv::sf::FileSystemType, tid: ::ipcdefs::nn::ApplicationId, path: &[u8; 0x301]) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			filesystem_type: ::ipcdefs::nn::fssrv::sf::FileSystemType,
			tid: ::ipcdefs::nn::ApplicationId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(0)
			.args(InRaw {
				filesystem_type,
				tid,
			})
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn initialize(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_data_file_system_by_current_process(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn open_file_system_with_patch(&self, filesystem_type: ::ipcdefs::nn::fssrv::sf::FileSystemType, tid: ::ipcdefs::nn::ApplicationId) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			filesystem_type: ::ipcdefs::nn::fssrv::sf::FileSystemType,
			tid: ::ipcdefs::nn::ApplicationId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(7)
			.args(InRaw {
				filesystem_type,
				tid,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn open_file_system_with_id(&self, filesystem_type: ::ipcdefs::nn::fssrv::sf::FileSystemType, tid: ::ipcdefs::nn::ApplicationId, path: &[u8; 0x301]) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			filesystem_type: ::ipcdefs::nn::fssrv::sf::FileSystemType,
			tid: ::ipcdefs::nn::ApplicationId,
		}
		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(8)
			.args(InRaw {
				filesystem_type,
				tid,
			})
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn open_data_file_system_by_application_id(&self, tid: ::ipcdefs::nn::ApplicationId) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(9)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_bis_file_system(&self, partition_id: ::ipcdefs::nn::fssrv::sf::Partition, unk1: &[u8; 0x301]) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(11)
			.args(partition_id)
			.descriptor(IPCBuffer::from_ref(unk1, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_bis_storage(&self, partition_id: ::ipcdefs::nn::fssrv::sf::Partition) -> Result<::ipcdefs::nn::fssrv::sf::IStorage<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(12)
			.args(partition_id)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn invalidate_bis_cache(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_host_file_system(&self, path: &[u8; 0x301]) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(17)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_sd_card_file_system(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(18)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn format_sd_card_file_system(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(19)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_save_data_file_system(&self, tid: ::ipcdefs::nn::ApplicationId) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(21)
			.args(tid)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_save_data_file_system(&self, save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct, save_create: ::ipcdefs::nn::fssrv::sf::SaveCreateStruct, input: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct,
			save_create: ::ipcdefs::nn::fssrv::sf::SaveCreateStruct,
			input: u128,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(22)
			.args(InRaw {
				save_struct,
				save_create,
				input,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_save_data_file_system_by_system_save_data_id(&self, save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct, save_create: ::ipcdefs::nn::fssrv::sf::SaveCreateStruct) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct,
			save_create: ::ipcdefs::nn::fssrv::sf::SaveCreateStruct,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(23)
			.args(InRaw {
				save_struct,
				save_create,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn register_save_data_file_system_atomic_deletion(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn delete_save_data_file_system_by_save_data_space_id(&self, unk0: u8, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(25)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn format_sd_card_dry_run(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(26)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn is_ex_fat_supported(&self, ) -> Result<u8> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(27)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn delete_save_data_file_system_by_save_data_attribute(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(28)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_game_card_storage(&self, partition_id: ::ipcdefs::nn::fssrv::sf::Partition, unk1: u32) -> Result<::ipcdefs::nn::fssrv::sf::IStorage<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			partition_id: ::ipcdefs::nn::fssrv::sf::Partition,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(30)
			.args(InRaw {
				partition_id,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_game_card_file_system(&self, unk0: u32, unk1: u32) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(31)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn extend_save_data_file_system(&self, unk0: u8, unk1: u64, unk2: u64, unk3: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
			unk2: u64,
			unk3: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(32)
			.args(InRaw {
				unk0,
				unk1,
				unk2,
				unk3,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn delete_cache_storage(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(33)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_cache_storage_size(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(34)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_save_data_file_system(&self, save_data_space_id: u8, save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			save_data_space_id: u8,
			save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(51)
			.args(InRaw {
				save_data_space_id,
				save_struct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_save_data_file_system_by_system_save_data_id(&self, save_data_space_id: u8, save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			save_data_space_id: u8,
			save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(52)
			.args(InRaw {
				save_data_space_id,
				save_struct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn open_read_only_save_data_file_system(&self, save_data_space_id: u8, save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			save_data_space_id: u8,
			save_struct: ::ipcdefs::nn::fssrv::sf::SaveStruct,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(53)
			.args(InRaw {
				save_data_space_id,
				save_struct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	// fn read_save_data_file_system_extra_data_by_save_data_space_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_save_data_file_system_extra_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_save_data_file_system_extra_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn open_save_data_info_reader(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::ISaveDataInfoReader<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(60)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_save_data_info_reader_by_save_data_space_id(&self, unk0: u8) -> Result<::ipcdefs::nn::fssrv::sf::ISaveDataInfoReader<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(61)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn open_cache_storage_list(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(62)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn open_save_data_internal_storage_file_system(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(64)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn update_save_data_mac_for_debug(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(65)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn write_save_data_file_system_extra_data2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(66)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn open_save_data_meta_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-4.0.0")]
	pub fn open_save_data_transfer_manager(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(81)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn open_save_data_transfer_manager_version2(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(82)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_image_directory_file_system(&self, unk0: u32) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_content_storage_file_system(&self, content_storage_id: u32) -> Result<::ipcdefs::nn::fssrv::sf::IFileSystem<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(110)
			.args(content_storage_id)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_data_storage_by_current_process(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::IStorage<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(200)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn open_data_storage_by_program_id(&self, tid: ::ipcdefs::nn::ApplicationId) -> Result<::ipcdefs::nn::fssrv::sf::IStorage<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(201)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_data_storage_by_data_id(&self, storage_id: u8, tid: ::ipcdefs::nn::ApplicationId) -> Result<::ipcdefs::nn::fssrv::sf::IStorage<T>> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			storage_id: u8,
			tid: ::ipcdefs::nn::ApplicationId,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(202)
			.args(InRaw {
				storage_id,
				tid,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_patch_data_storage_by_current_process(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::IStorage<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_device_operator(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::IDeviceOperator<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(400)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_sd_card_detection_event_notifier(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::IEventNotifier<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(500)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	pub fn open_game_card_detection_event_notifier(&self, ) -> Result<::ipcdefs::nn::fssrv::sf::IEventNotifier<T>> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(501)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(T::from_res(&mut res).into())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn open_system_data_update_event_notifier(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(510)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn notify_system_data_update_event(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(511)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(not(feature = "switch-4.0.0"))]
	pub fn set_current_posix_time(&self, time: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(600)
			.args(time)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn query_save_data_total_size(&self, unk0: u64, unk1: u64) -> Result<u64> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(601)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn verify_save_data_file_system(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn corrupt_save_data_file_system(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(603)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_padding_file(&self, unk0: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(604)
			.args(unk0)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_all_padding_files(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(605)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_rights_id(&self, unk0: u8, unk1: u64) -> Result<u128> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(606)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn register_external_key(&self, unk0: u128, unk1: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u128,
			unk1: u128,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(607)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn unregister_all_external_key(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(608)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_rights_id_by_path(&self, path: &[u8; 0x301]) -> Result<u128> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(609)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn get_rights_id_and_key_generation_by_path(&self, path: &[u8; 0x301]) -> Result<(u8, u128)> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(610)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		#[repr(C)] #[derive(Clone)] struct OutRaw {
			unk1: u8,
			rights: u128,
		}
		let res : Response<OutRaw> = self.0.send(req)?;
		Ok((res.get_raw().unk1.clone(),res.get_raw().rights.clone()))
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_current_posix_time_with_time_difference(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(611)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_free_space_size_for_save_data(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(612)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn verify_save_data_file_system_by_save_data_space_id(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(613)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn corrupt_save_data_file_system_by_save_data_space_id(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(614)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn query_save_data_internal_storage_total_size(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(615)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn set_sd_card_encryption_seed(&self, seed: u128) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(620)
			.args(seed)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn set_sd_card_accessibility(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(630)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn is_sd_card_accessible(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(631)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn is_signed_system_partition_on_sd_card_valid(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(640)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn open_access_failure_resolver(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(700)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn get_access_failure_detection_event(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(701)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn is_access_failure_detected(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(702)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn resolve_access_failure(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(710)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.0.0")]
	pub fn abandon_access_failure(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(720)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_and_clear_file_system_proxy_error_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_bis_root_for_host(&self, unk0: u32, path: &[u8; 0x301]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1000)
			.args(unk0)
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_save_data_size(&self, unk0: u64, unk1: u64) -> Result<()> {
		use ::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1001)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_save_data_root_path(&self, path: &[u8; 0x301]) -> Result<()> {
		use ::ipc::IPCBuffer;
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 1], [_; 0], [_; 0]> = Request::new(1002)
			.args(())
			.descriptor(IPCBuffer::from_ref(path, 0x19))
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn disable_auto_save_data_creation(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1003)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_global_access_log_mode(&self, mode: u32) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1004)
			.args(mode)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_global_access_log_mode(&self, ) -> Result<u32> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1005)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn output_access_log_to_sd_card(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-4.0.0")]
	pub fn register_update_partition(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1007)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn open_registered_update_partition(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1008)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn get_and_clear_memory_report_info(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1009)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-5.1.0")]
	pub fn unknown1010(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1010)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-4.0.0")]
	pub fn override_save_data_transfer_token_sign_verification_key(&self, ) -> Result<()> {
		use ::ipc::{Request, Response};

		let req : Request<_, [_; 0], [_; 0], [_; 0]> = Request::new(1100)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

}

impl<T: Object> From<T> for IFileSystemProxy<T> {
	fn from(obj: T) -> IFileSystemProxy<T> {
		IFileSystemProxy(obj)
	}
}
