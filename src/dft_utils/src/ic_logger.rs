use ic_cdk::api;
use log::{Level, LevelFilter, Metadata, Record};
use yansi::Paint;

pub struct ICLogger;

impl log::Log for ICLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();
            let str = match level {
                Level::Error => Paint::red(format!("{} - {}", level, record.args())),
                Level::Warn => Paint::yellow(format!("{} - {}", level, record.args())),
                Level::Info => Paint::blue(format!("{} - {}", level, record.args())),
                Level::Debug => Paint::green(format!("{} - {}", level, record.args())),
                Level::Trace => Paint::magenta(format!("{} - {}", level, record.args())),
            };
            api::print(str.to_string());
        }
    }

    fn flush(&self) {}
}

impl ICLogger {
    pub fn init() {
        #[cfg(feature = "dev_canister")]
        {
            match log::set_logger(&ICLogger) {
                Ok(_) => {
                    log::set_max_level(LevelFilter::Trace);
                }
                Err(_) => {}
            };
        }
    }
}
