#[macro_use]
extern crate log;

mod auth;
mod errors;
pub mod github_client;
mod query_helper;
pub mod twitter_client;

/// a simple logger that prints to stdout -
/// this is the way that Heroku works, but
/// better loggers can be plugged in depending
/// on the hosting option
mod lib_logger {
    use log::{Level, Metadata, Record};

    pub struct ConsoleLogger;

    impl log::Log for ConsoleLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Info
        }

        fn log(&self, record: &Record) {
            println!("{}- {}", record.level(), record.args());
        }

        fn flush(&self) {}
    }
}

use lib_logger::ConsoleLogger;
use log::{LevelFilter, SetLoggerError};

static LOGGER: ConsoleLogger = ConsoleLogger;

fn init_lib_logging() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|_| log::set_max_level(LevelFilter::Info))
}

/// public interface to the twitter_client module function
pub fn start_twitter_service() {
    init_lib_logging().unwrap_or_else(|_| {});
    twitter_client::start_service();
}
