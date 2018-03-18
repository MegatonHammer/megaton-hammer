
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use alloc::arc::Arc;

#[derive(Debug)]
pub struct IFileSystemProxy(Session);

impl IFileSystemProxy {
	pub fn new() -> Result<Arc<IFileSystemProxy>> {
		use alloc::arc::Weak;
		use spin::Mutex;
		lazy_static! {
			static ref HANDLE : Mutex<Weak<IFileSystemProxy>> = Mutex::new(Weak::new());
		}
		if let Some(hnd) = HANDLE.lock().upgrade() {
			return Ok(hnd)
		}
		use nn::sm::detail::IUserInterface;

		let sm = IUserInterface::new()?;

		let r = sm.get_service(*b"fsp-srv\0").map(|s| Arc::new(unsafe { IFileSystemProxy::from_kobject(s) }));
		if let Ok(service) = r {
			*HANDLE.lock() = Arc::downgrade(&service);
			return Ok(service);
		}
		r
	}
}

impl AsRef<Session> for IFileSystemProxy {
	fn as_ref(&self) -> &Session {
		&self.0
	}
}
impl IFileSystemProxy {
	// fn mount_content(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn initialize(&self, unk0: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn open_data_file_system_by_current_process(&self, ) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn mount_content7(&self, tid: ::nn::ApplicationId, nca_type: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			tid: ::nn::ApplicationId,
			nca_type: u32,
		}
		let req = Request::new(7)
			.args(InRaw {
				tid,
				nca_type,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn mount_content(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn open_data_file_system_by_application_id(&self, tid: ::nn::ApplicationId) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(9)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn mount_bis(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn open_bis_partition(&self, partition_id: ::nn::fssrv::sf::Partition) -> Result<::nn::fssrv::sf::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(12)
			.args(partition_id)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn invalidate_bis_cache(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(13)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn open_host_file_system_impl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn mount_sd_card(&self, ) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn format_sd_card(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(19)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_save_data(&self, tid: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(21)
			.args(tid)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_save_data(&self, save_struct: ::nn::fssrv::sf::SaveStruct, save_create: ::nn::fssrv::sf::SaveCreateStruct, input: u128) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			save_struct: ::nn::fssrv::sf::SaveStruct,
			save_create: ::nn::fssrv::sf::SaveCreateStruct,
			input: u128,
		}
		let req = Request::new(22)
			.args(InRaw {
				save_struct,
				save_create,
				input,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_system_save_data(&self, save_struct: ::nn::fssrv::sf::SaveStruct, save_create: ::nn::fssrv::sf::SaveCreateStruct) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			save_struct: ::nn::fssrv::sf::SaveStruct,
			save_create: ::nn::fssrv::sf::SaveCreateStruct,
		}
		let req = Request::new(23)
			.args(InRaw {
				save_struct,
				save_create,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn register_save_data_atomic_deletion(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn delete_save_data_with_space_id(&self, unk0: u8, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
		}
		let req = Request::new(25)
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
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(26)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn is_ex_fat_supported(&self, ) -> Result<u8> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(27)
			.args(())
			;
		let res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn open_game_card_partition(&self, partition_id: ::nn::fssrv::sf::Partition, unk1: u32) -> Result<::nn::fssrv::sf::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			partition_id: ::nn::fssrv::sf::Partition,
			unk1: u32,
		}
		let req = Request::new(30)
			.args(InRaw {
				partition_id,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn mount_game_card_partition(&self, unk0: u32, unk1: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u32,
			unk1: u32,
		}
		let req = Request::new(31)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn extend_save_data(&self, unk0: u8, unk1: u64, unk2: u64, unk3: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u8,
			unk1: u64,
			unk2: u64,
			unk3: u64,
		}
		let req = Request::new(32)
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

	pub fn mount_save_data(&self, input: u8, save_struct: ::nn::fssrv::sf::SaveStruct) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			input: u8,
			save_struct: ::nn::fssrv::sf::SaveStruct,
		}
		let req = Request::new(51)
			.args(InRaw {
				input,
				save_struct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn mount_system_save_data(&self, input: u8, save_struct: ::nn::fssrv::sf::SaveStruct) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			input: u8,
			save_struct: ::nn::fssrv::sf::SaveStruct,
		}
		let req = Request::new(52)
			.args(InRaw {
				input,
				save_struct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn mount_save_data_read_only(&self, input: u8, save_struct: ::nn::fssrv::sf::SaveStruct) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			input: u8,
			save_struct: ::nn::fssrv::sf::SaveStruct,
		}
		let req = Request::new(53)
			.args(InRaw {
				input,
				save_struct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn read_save_data_file_system_extra_data_with_space_id(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn read_save_data_file_system_extra_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn write_save_data_file_system_extra_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn open_save_data_info_reader(&self, ) -> Result<::nn::fssrv::sf::ISaveDataInfoReader> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(60)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_save_data_iterator(&self, unk0: u8) -> Result<Session> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(61)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn open_save_data_thumbnail_file(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn mount_image_directory(&self, unk0: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn mount_content_storage(&self, content_storage_id: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(110)
			.args(content_storage_id)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_data_storage_by_current_process(&self, ) -> Result<::nn::fssrv::sf::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn open_data_storage_by_application_id(&self, tid: ::nn::ApplicationId) -> Result<::nn::fssrv::sf::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(201)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_data_storage_by_data_id(&self, tid: ::nn::ApplicationId, storage_id: u8) -> Result<::nn::fssrv::sf::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			tid: ::nn::ApplicationId,
			storage_id: u8,
		}
		let req = Request::new(202)
			.args(InRaw {
				tid,
				storage_id,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_rom_storage(&self, ) -> Result<::nn::fssrv::sf::IStorage> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_device_operator(&self, ) -> Result<::nn::fssrv::sf::IDeviceOperator> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(400)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_sd_card_detection_event_notifier(&self, ) -> Result<::nn::fssrv::sf::IEventNotifier> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(500)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn open_game_card_detection_event_notifier(&self, ) -> Result<::nn::fssrv::sf::IEventNotifier> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(501)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(not(feature = "switch-4.0.0"))]
	pub fn set_current_posix_time(&self, time: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(600)
			.args(time)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn query_save_data_total_size(&self, unk0: u64, unk1: u64) -> Result<u64> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(601)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn verify_save_data(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn corrupt_save_data_for_debug(&self, tid: ::nn::ApplicationId) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(603)
			.args(tid)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn create_padding_file(&self, size: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(604)
			.args(size)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn delete_all_padding_files(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(605)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn get_rights_id(&self, unk0: u64, unk1: u8) -> Result<u128> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u8,
		}
		let req = Request::new(606)
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
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u128,
			unk1: u128,
		}
		let req = Request::new(607)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn unregister_external_key(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(608)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_rights_id_by_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn get_rights_id_by_path2(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn set_sd_card_encryption_seed(&self, seedmaybe: u128) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(620)
			.args(seedmaybe)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn get_and_clear_file_system_proxy_error_info(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn set_bis_root_for_host(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn set_save_data_size(&self, unk0: u64, unk1: u64) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			unk0: u64,
			unk1: u64,
		}
		let req = Request::new(1001)
			.args(InRaw {
				unk0,
				unk1,
			})
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn set_save_data_root_path(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn disable_auto_save_data_creation(&self, ) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1003)
			.args(())
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn set_global_access_log_mode(&self, mode: u32) -> Result<()> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1004)
			.args(mode)
			;
		let _res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn get_global_access_log_mode(&self, ) -> Result<u32> {
		use megaton_hammer::ipc::{Request, Response};

		let req = Request::new(1005)
			.args(())
			;
		let res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn output_access_log_to_sd_card(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFileSystemProxy {
	unsafe fn from_kobject(obj: KObject) -> IFileSystemProxy {
		IFileSystemProxy(Session::from_kobject(obj))
	}
}
