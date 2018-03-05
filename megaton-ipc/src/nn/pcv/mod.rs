mod impl_IArbitrationManager;
pub use self::impl_IArbitrationManager::*;
mod impl_IImmediateManager;
pub use self::impl_IImmediateManager::*;
pub mod detail;
pub type ModuleState = [u8; 0xc];
pub type PowerControlTarget = u32;
pub type TemperatureThreshold = ();
pub type PowerDomainState = ();
