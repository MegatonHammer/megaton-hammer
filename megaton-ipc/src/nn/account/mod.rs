pub mod http;
mod impl_IAccountServiceForSystemService;
pub use self::impl_IAccountServiceForSystemService::*;
pub mod baas;
pub mod profile;
mod impl_IAccountServiceForAdministrator;
pub use self::impl_IAccountServiceForAdministrator::*;
mod impl_IAccountServiceForApplication;
pub use self::impl_IAccountServiceForApplication::*;
pub mod nas;
pub mod detail;
mod impl_IBaasAccessTokenAccessor;
pub use self::impl_IBaasAccessTokenAccessor::*;
pub type NintendoAccountAuthorizationRequestParameters = ();
pub type RequestUrl = ();
pub type Nickname = [u8; 0x21];
pub type SystemProgramIdentification = ();
pub type Uid = u128;
pub type ProfileDigest = u128;
pub type NetworkServiceAccountId = u64;
pub type CallbackUri = ();
pub type NintendoAccountId = u64;
