use ic_cdk::api;
use log::{Level, LevelFilter, Metadata, Record};
use std::panic;
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
        #[cfg(feature = "logger")]
        {
            if log::set_logger(&ICLogger).is_ok() {
                log::set_max_level(LevelFilter::Trace);
                panic::set_hook(Box::new(|data| {
                    let message = format!("{}", data);
                    api::print(Paint::red(message).to_string());
                }));
            };
        }
    }
}

pub fn init_test_logger() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Trace)
        .is_test(true)
        .try_init();
}
