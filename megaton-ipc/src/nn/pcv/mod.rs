mod impl_i_arbitration_manager;
pub use self::impl_i_arbitration_manager::*;
mod impl_i_immediate_manager;
pub use self::impl_i_immediate_manager::*;
pub mod detail;
pub type ModuleState = [u8; 0xc];
pub type PowerControlTarget = u32;
pub type TemperatureThreshold = ();
pub type PowerDomainState = ();
