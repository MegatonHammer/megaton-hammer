mod impl_INotificationService;
pub use self::impl_INotificationService::*;
mod impl_IServiceCreator;
pub use self::impl_IServiceCreator::*;
mod impl_IFriendService;
pub use self::impl_IFriendService::*;
pub type SizedFriendFilter = u128;
pub type SizedNotificationInfo = u128;
