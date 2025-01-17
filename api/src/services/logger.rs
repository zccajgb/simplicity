use chrono::Local;

pub struct TimestampLogger;

impl log::Log for TimestampLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
            println!("{} [{}] - {}", timestamp, record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
