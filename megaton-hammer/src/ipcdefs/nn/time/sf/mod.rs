#[repr(C)]
#[derive(Clone, Copy)]
pub struct CalendarAdditionalInfo {
	pub tm_wday: u32,
	pub tm_yday: i32,
	pub tz_name: [u8; 8],
	pub is_daylight_saving_time: bool,
	pub utc_offset_seconds: i32,
}
