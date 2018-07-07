use log::{self, Log, Metadata, Record, LevelFilter};
use spin::Once;
use core::fmt::Write;

struct Logger;

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let _ = writeln!(::loader::Logger, "[{}] - {} - {}", record.level(), record.target(), record.args());
    }

    fn flush(&self) {}
}

static LOGGER: Once<Logger> = Once::new();

pub fn init() {
    log::set_logger(LOGGER.call_once(|| Logger))
        .expect("log_impl::init to be called only once");
    log::set_max_level(LevelFilter::Trace);
    info!("Logging enabled");
}
