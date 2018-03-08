pub type SystemClockContext = [u8; 0x20];
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CalendarTime {
	year: u16,
	month: u8,
	day: u8,
	hour: u8,
	minute: u8,
	second: u8,
}
pub type PosixTime = u64;
pub type TimeZoneRuleVersion = u128;
pub type TimeZoneRule = ();
pub mod sf;
pub type LocationName = [u8; 0x24];
pub type SteadyClockTimePoint = [u8; 0x18];
