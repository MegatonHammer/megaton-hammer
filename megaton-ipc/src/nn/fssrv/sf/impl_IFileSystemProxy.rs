
use megaton_hammer::kernel::{FromKObject, KObject, Session};
use megaton_hammer::error::Result;
use megaton_hammer::ipc::{Request, Response};
use megaton_hamer::ipc::IPCBuffer

#[derive(Debug)]
pub struct IFileSystemProxy(Session);

impl IFileSystemProxy {
	pub fn get_service() -> Result<IFileSystemProxy> {
		use nn::sm::detail::IUserInterface;
		use megaton_hammer::kernel::svc;
		use megaton_hammer::error::Error;

		let sm = IUserInterface::get_service()?;
		let r = sm.GetService(*b"fsp-srv\0").map(|s| unsafe { IFileSystemProxy::from_kobject(s) });
		if let Ok(service) = r {
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
	// fn MountContent(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn Initialize(&self, unk0: u64) -> Result<()> {
		let req = Request::new(1)
			.args(unk0)
			.send_pid()
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn OpenDataFileSystemByCurrentProcess(&self, ) -> Result<::nn::fssrv::sf::IFileSystem> {
		let req = Request::new(2)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn MountContent7(&self, tid: ::nn::ApplicationId, ncaType: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			tid: ::nn::ApplicationId,
			ncaType: u32,
		}
		let req = Request::new(7)
			.args(InRaw {
				tid,
				ncaType,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn MountContent(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-3.0.0")]
	pub fn OpenDataFileSystemByApplicationId(&self, tid: ::nn::ApplicationId) -> Result<::nn::fssrv::sf::IFileSystem> {
		let req = Request::new(9)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn MountBis(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn OpenBisPartition(&self, partitionID: ::nn::fssrv::sf::Partition) -> Result<::nn::fssrv::sf::IStorage> {
		let req = Request::new(12)
			.args(partitionID)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn InvalidateBisCache(&self, ) -> Result<()> {
		let req = Request::new(13)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn OpenHostFileSystemImpl(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn MountSdCard(&self, ) -> Result<::nn::fssrv::sf::IFileSystem> {
		let req = Request::new(18)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn FormatSdCard(&self, ) -> Result<()> {
		let req = Request::new(19)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteSaveData(&self, tid: ::nn::ApplicationId) -> Result<()> {
		let req = Request::new(21)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateSaveData(&self, saveStruct: ::nn::fssrv::sf::SaveStruct, saveCreate: ::nn::fssrv::sf::SaveCreateStruct, input: u128) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			saveStruct: ::nn::fssrv::sf::SaveStruct,
			saveCreate: ::nn::fssrv::sf::SaveCreateStruct,
			input: u128,
		}
		let req = Request::new(22)
			.args(InRaw {
				saveStruct,
				saveCreate,
				input,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreateSystemSaveData(&self, saveStruct: ::nn::fssrv::sf::SaveStruct, saveCreate: ::nn::fssrv::sf::SaveCreateStruct) -> Result<()> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			saveStruct: ::nn::fssrv::sf::SaveStruct,
			saveCreate: ::nn::fssrv::sf::SaveCreateStruct,
		}
		let req = Request::new(23)
			.args(InRaw {
				saveStruct,
				saveCreate,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn RegisterSaveDataAtomicDeletion(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn DeleteSaveDataWithSpaceId(&self, unk0: u8, unk1: u64) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn FormatSdCardDryRun(&self, ) -> Result<()> {
		let req = Request::new(26)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn IsExFatSupported(&self, ) -> Result<u8> {
		let req = Request::new(27)
			.args(())
			;
		let mut res : Response<u8> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	pub fn OpenGameCardPartition(&self, partitionID: ::nn::fssrv::sf::Partition, unk1: u32) -> Result<::nn::fssrv::sf::IStorage> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			partitionID: ::nn::fssrv::sf::Partition,
			unk1: u32,
		}
		let req = Request::new(30)
			.args(InRaw {
				partitionID,
				unk1,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn MountGameCardPartition(&self, unk0: u32, unk1: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
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
	pub fn ExtendSaveData(&self, unk0: u8, unk1: u64, unk2: u64, unk3: u64) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn MountSaveData(&self, input: u8, saveStruct: ::nn::fssrv::sf::SaveStruct) -> Result<::nn::fssrv::sf::IFileSystem> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			input: u8,
			saveStruct: ::nn::fssrv::sf::SaveStruct,
		}
		let req = Request::new(51)
			.args(InRaw {
				input,
				saveStruct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn MountSystemSaveData(&self, input: u8, saveStruct: ::nn::fssrv::sf::SaveStruct) -> Result<::nn::fssrv::sf::IFileSystem> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			input: u8,
			saveStruct: ::nn::fssrv::sf::SaveStruct,
		}
		let req = Request::new(52)
			.args(InRaw {
				input,
				saveStruct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn MountSaveDataReadOnly(&self, input: u8, saveStruct: ::nn::fssrv::sf::SaveStruct) -> Result<::nn::fssrv::sf::IFileSystem> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			input: u8,
			saveStruct: ::nn::fssrv::sf::SaveStruct,
		}
		let req = Request::new(53)
			.args(InRaw {
				input,
				saveStruct,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn ReadSaveDataFileSystemExtraDataWithSpaceId(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn ReadSaveDataFileSystemExtraData(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn WriteSaveDataFileSystemExtraData(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn OpenSaveDataInfoReader(&self, ) -> Result<::nn::fssrv::sf::ISaveDataInfoReader> {
		let req = Request::new(60)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenSaveDataIterator(&self, unk0: u8) -> Result<Session> {
		let req = Request::new(61)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	// fn OpenSaveDataThumbnailFile(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn MountImageDirectory(&self, unk0: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
		let req = Request::new(100)
			.args(unk0)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn MountContentStorage(&self, contentStorageID: u32) -> Result<::nn::fssrv::sf::IFileSystem> {
		let req = Request::new(110)
			.args(contentStorageID)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenDataStorageByCurrentProcess(&self, ) -> Result<::nn::fssrv::sf::IStorage> {
		let req = Request::new(200)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(feature = "switch-3.0.0")]
	pub fn OpenDataStorageByApplicationId(&self, tid: ::nn::ApplicationId) -> Result<::nn::fssrv::sf::IStorage> {
		let req = Request::new(201)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenDataStorageByDataId(&self, tid: ::nn::ApplicationId, storageId: u8) -> Result<::nn::fssrv::sf::IStorage> {
		#[repr(C)] #[derive(Clone)]
		struct InRaw {
			tid: ::nn::ApplicationId,
			storageId: u8,
		}
		let req = Request::new(202)
			.args(InRaw {
				tid,
				storageId,
			})
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenRomStorage(&self, ) -> Result<::nn::fssrv::sf::IStorage> {
		let req = Request::new(203)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenDeviceOperator(&self, ) -> Result<::nn::fssrv::sf::IDeviceOperator> {
		let req = Request::new(400)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenSdCardDetectionEventNotifier(&self, ) -> Result<::nn::fssrv::sf::IEventNotifier> {
		let req = Request::new(500)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	pub fn OpenGameCardDetectionEventNotifier(&self, ) -> Result<::nn::fssrv::sf::IEventNotifier> {
		let req = Request::new(501)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(unsafe { FromKObject::from_kobject(res.pop_handle()) })
	}

	#[cfg(not(feature = "switch-4.0.0"))]
	pub fn SetCurrentPosixTime(&self, time: u64) -> Result<()> {
		let req = Request::new(600)
			.args(time)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn QuerySaveDataTotalSize(&self, unk0: u64, unk1: u64) -> Result<u64> {
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
		let mut res : Response<u64> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn VerifySaveData(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn CorruptSaveDataForDebug(&self, tid: ::nn::ApplicationId) -> Result<()> {
		let req = Request::new(603)
			.args(tid)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn CreatePaddingFile(&self, size: u64) -> Result<()> {
		let req = Request::new(604)
			.args(size)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn DeleteAllPaddingFiles(&self, ) -> Result<()> {
		let req = Request::new(605)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn GetRightsId(&self, unk0: u64, unk1: u8) -> Result<u128> {
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
		let mut res : Response<u128> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn RegisterExternalKey(&self, unk0: u128, unk1: u128) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	#[cfg(feature = "switch-2.0.0")]
	pub fn UnregisterExternalKey(&self, ) -> Result<()> {
		let req = Request::new(608)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetRightsIdByPath(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn GetRightsIdByPath2(&self, UNKNOWN) -> Result<UNKNOWN>;
	#[cfg(feature = "switch-2.0.0")]
	pub fn SetSdCardEncryptionSeed(&self, seedmaybe: u128) -> Result<()> {
		let req = Request::new(620)
			.args(seedmaybe)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn GetAndClearFileSystemProxyErrorInfo(&self, UNKNOWN) -> Result<UNKNOWN>;
	// fn SetBisRootForHost(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn SetSaveDataSize(&self, unk0: u64, unk1: u64) -> Result<()> {
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
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	// fn SetSaveDataRootPath(&self, UNKNOWN) -> Result<UNKNOWN>;
	pub fn DisableAutoSaveDataCreation(&self, ) -> Result<()> {
		let req = Request::new(1003)
			.args(())
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn SetGlobalAccessLogMode(&self, mode: u32) -> Result<()> {
		let req = Request::new(1004)
			.args(mode)
			;
		let mut res : Response<()> = self.0.send(req)?;
		Ok(())
	}

	pub fn GetGlobalAccessLogMode(&self, ) -> Result<u32> {
		let req = Request::new(1005)
			.args(())
			;
		let mut res : Response<u32> = self.0.send(req)?;
		Ok(*res.get_raw())
	}

	// fn OutputAccessLogToSdCard(&self, UNKNOWN) -> Result<UNKNOWN>;
}

impl FromKObject for IFileSystemProxy {
	unsafe fn from_kobject(obj: KObject) -> IFileSystemProxy {
		IFileSystemProxy(Session::from_kobject(obj))
	}
}
