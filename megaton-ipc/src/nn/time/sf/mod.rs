#[repr(C)]
#[derive(Debug, Clone)]
pub struct CalendarAdditionalInfo {
	tm_wday: u32,
	tm_yday: i32,
	tz_name: [u8; 8],
	is_daylight_saving_time: bool,
	utc_offset_seconds: i32,
}
