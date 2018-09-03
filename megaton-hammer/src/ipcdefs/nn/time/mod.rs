pub type SystemClockContext = [u8; 0x20];
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CalendarTime {
	pub year: u16,
	pub month: u8,
	pub day: u8,
	pub hour: u8,
	pub minute: u8,
	pub second: u8,
}
pub type TimeZoneRuleVersion = u128;
pub type PosixTime = u64;
pub mod sf;
pub type LocationName = [u8; 0x24];
pub type TimeZoneRule = ();
pub type SteadyClockTimePoint = [u8; 0x18];
