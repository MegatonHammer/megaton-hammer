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
pub type SaveCreateStruct = [u8; 0x40];
pub type IDirectoryEntry = [u8; 0x310];
pub type Partition = u32;