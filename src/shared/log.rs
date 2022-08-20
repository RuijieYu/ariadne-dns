use self::Level as LogLevel;
pub use log::Level;
use simple_logger::SimpleLogger;

// // This remains because log::Level does not implement Ser/De
// #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialOrd, PartialEq)]
// pub enum LogLevel {
//     Debug,
//     Info,
//     Warn,
//     Error,
// }

// impl LogLevel {
//     pub const fn level(self) -> Level {
//         match self {
//             LogLevel::Debug => Level::Debug,
//             LogLevel::Info => Level::Info,
//             LogLevel::Warn => Level::Warn,
//             LogLevel::Error => Level::Error,
//         }
//     }
// }

/// Initialize the logging facility with Debug level.
pub fn init_log() {
    SimpleLogger::new()
        .with_level(Level::Debug.to_level_filter())
        .init()
        .unwrap()
}

#[inline]
pub fn set_max_level(lvl: LogLevel) {
    ::log::set_max_level(lvl.to_level_filter())
}

/// Retrieving the logging level is no longer necessary or possible.
pub const fn log_level() -> LogLevel {
    panic!("log_level() is no longer available")
}
