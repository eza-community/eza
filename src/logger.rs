//! Debug error logging.

use std::ffi::OsStr;

use nu_ansi_term::{Color, AnsiString};


/// Sets the internal logger, changing the log level based on the value of an
/// environment variable.
pub fn configure<T: AsRef<OsStr>>(ev: Option<T>) {
    let Some(ev) = ev else { return };

    let env_var = ev.as_ref();
    if env_var.is_empty() {
        return;
    }

    if env_var == "trace" {
        log::set_max_level(log::LevelFilter::Trace);
    }
    else {
        log::set_max_level(log::LevelFilter::Debug);
    }

    let result = log::set_logger(GLOBAL_LOGGER);
    if let Err(e) = result {
        eprintln!("Failed to initialise logger: {e}");
    }
}


#[derive(Debug)]
struct Logger;

const GLOBAL_LOGGER: &Logger = &Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true  // no need to filter after using ‘set_max_level’.
    }

    fn log(&self, record: &log::Record<'_>) {
        let open = Color::Fixed(243).paint("[");
        let level = level(record.level());
        let close = Color::Fixed(243).paint("]");

        eprintln!("{}{} {}{} {}", open, level, record.target(), close, record.args());
    }

    fn flush(&self) {
        // no need to flush with ‘eprintln!’.
    }
}

fn level(level: log::Level) -> AnsiString<'static> {
    match level {
        log::Level::Error  => Color::Red.paint("ERROR"),
        log::Level::Warn   => Color::Yellow.paint("WARN"),
        log::Level::Info   => Color::Cyan.paint("INFO"),
        log::Level::Debug  => Color::Blue.paint("DEBUG"),
        log::Level::Trace  => Color::Fixed(245).paint("TRACE"),
    }
}
