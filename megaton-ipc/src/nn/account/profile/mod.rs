mod impl_IProfile;
pub use self::impl_IProfile::*;
mod impl_IProfileEditor;
pub use self::impl_IProfileEditor::*;
pub type UserData = ();
pub type ProfileBase = [u8; 0x38];
