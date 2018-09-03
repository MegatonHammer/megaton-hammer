mod impl_i_file;
pub use self::impl_i_file::*;
mod impl_i_event_notifier;
pub use self::impl_i_event_notifier::*;
mod impl_i_program_registry;
pub use self::impl_i_program_registry::*;
mod impl_i_storage;
pub use self::impl_i_storage::*;
mod impl_i_save_data_info_reader;
pub use self::impl_i_save_data_info_reader::*;
mod impl_i_directory;
pub use self::impl_i_directory::*;
mod impl_i_file_system_proxy_for_loader;
pub use self::impl_i_file_system_proxy_for_loader::*;
mod impl_i_device_operator;
pub use self::impl_i_device_operator::*;
mod impl_i_file_system_proxy;
pub use self::impl_i_file_system_proxy::*;
mod impl_i_file_system;
pub use self::impl_i_file_system::*;
pub type SaveStruct = [u8; 0x40];
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum FileSystemType {
Invalid = 0,
Invalid2 = 1,
Logo = 2,
ContentControl = 3,
ContentManual = 4,
ContentMeta = 5,
ContentData = 6,
ApplicationPackage = 7,
}
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DirectoryEntryType {
Directory = 0,
File = 1,
}
pub type SaveCreateStruct = [u8; 0x40];
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IDirectoryEntry {
	pub path: [u8; 0x300],
	pub unk1: u32,
	pub directory_entry_type: ::ipcdefs::nn::fssrv::sf::DirectoryEntryType,
	pub filesize: u64,
}
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Partition {
BootPartition1Root = 0,
BootPartition2Root = 10,
UserDataRoot = 20,
BootConfigAndPackage2Part1 = 21,
BootConfigAndPackage2Part2 = 22,
BootConfigAndPackage2Part3 = 23,
BootConfigAndPackage2Part4 = 24,
BootConfigAndPackage2Part5 = 25,
BootConfigAndPackage2Part6 = 26,
CalibrationBinary = 27,
CalibrationFile = 28,
SafeMode = 29,
SystemProperEncryption = 30,
User = 31,
}
