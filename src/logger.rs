use crate::console::{print_colorize, ANSICON};
use log::{Level, LevelFilter, Metadata, Record};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG"){
        Some("ERROR") => LevelFilter::Error,
        Some("WARN") => LevelFilter::Warn,
        Some("INFO") => LevelFilter::Info,
        Some("DEBUG") => LevelFilter::Debug,
        Some("TRACE") => LevelFilter::Trace,
        _ => LevelFilter::Off,
    });
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print_colorize(
                // https://doc.rust-lang.org/std/fmt/ 格式化参数 < 右对齐 ^居中 >左对齐
                format_args!("[{:^5}] : {}\n", record.level(), record.args()),
                level_to_color(record.level()),
                ANSICON::BgDefault as u8
            )
        }
    }

    fn flush(&self) {}
}
fn level_to_color(level: Level) -> u8 {
    match level {
        Level::Error => ANSICON::FgRed as u8,
        Level::Warn => ANSICON::FgLightYellow as u8,
        Level::Info => ANSICON::FgBlue as u8,
        Level::Debug => ANSICON::FgGreen as u8,
        Level::Trace => ANSICON::FgLightGray as u8,
    }
}