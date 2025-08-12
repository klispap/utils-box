//! # Logger utilities
//! A toolbox of small utilities to initialize and use loggers based on `simplelog` crate.
//! Useful for binaries that you need a terminal or a file logger fast.

use std::{fs::File, path::PathBuf, str::FromStr, sync::Mutex};

use chrono;
use file_rotate::{ContentLimit, FileRotate, compression::Compression, suffix::AppendCount};
use lazy_static::lazy_static;
use log::info;
use simplelog::*;

/// Initialize a terminal logger with the provided log level
pub fn terminal_logger_init(level: LevelFilter) {
    // Initialize the loggers
    let log_config = ConfigBuilder::new()
        .set_time_format("%d-%H:%M:%S%.3f".to_string())
        .set_time_to_local(true)
        .build();

    TermLogger::init(
        level,
        log_config.clone(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Logger could not be initialized! Other logger is already active!");
}

// Initialize a terminal and a file logger with the provided log levels
pub fn combined_logger_init(
    terminal_level: LevelFilter,
    file_level: LevelFilter,
    log_path: &str,
    filename_prefix: &str,
) {
    let mut log_path = if log_path.is_empty() {
        std::env::temp_dir()
    } else {
        PathBuf::from_str(log_path).unwrap()
    };

    let instance_folder = format!("{}_{:?}", filename_prefix, chrono::offset::Utc::now());
    log_path.push(format!("{filename_prefix}-logs"));
    log_path.push(&instance_folder);
    log_path.push(format!("{filename_prefix}.log"));

    println!("[logger_init] Calculated log path [{log_path:?}]");

    let log_config = ConfigBuilder::new()
        .set_time_format("%d-%H:%M:%S%.3f".to_string())
        .set_time_to_local(true)
        .build();
    CombinedLogger::init(vec![
        TermLogger::new(
            terminal_level,
            log_config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            file_level,
            log_config,
            FileRotate::new(
                log_path.clone(),
                AppendCount::new(5),
                ContentLimit::Lines(30000),
                Compression::None,
                #[cfg(unix)]
                None,
            ),
        ),
    ])
    .expect("Logger could not be initialized! Other logger is already active!");

    info!("[logger_init] Calculated log path [{log_path:?}]");
}

lazy_static! {
    pub static ref DUMMY: log::Metadata<'static> = log::MetadataBuilder::new().build();

    /// Scanner Results File (Used for Demos)
    pub static ref RESULTS_FILE: Mutex<File> = {
        let mut log_path = std::env::temp_dir();
        log_path.push("utils-results.log");

        Mutex::new(File::options()
            .append(true)
            .create(true)
            .open(log_path)
            .expect("Results File FAILED!"))
    };
}

pub enum LoggerPrintMode {
    Results,
    Info,
}

#[macro_export]
macro_rules! results_info {
    (mode:$mode:expr, $($arg:tt)+) => {{
        if $mode == "results" {
            $crate::results_info!($($arg)+);
        }
        else {
            $crate::results_info!(info, $($arg)+);
        }
    }};

    (info, $($arg:tt)+) => {{
        $crate::log_info!($($arg)+);
    }};

    ($($arg:tt)+) => {{
        use std::io::Write;
        use chrono::Local;

        $crate::log_info!($($arg)+);

        let timestamp = Local::now().format("%d-%H:%M:%S%.3f").to_string();
        let mut res_file = $crate::logger::RESULTS_FILE.lock().unwrap();
        writeln!(res_file,"{} {}",timestamp, format!($($arg)+)).unwrap();
    }};
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => {{
        if log::logger().enabled(&$crate::logger::DUMMY) {
            log::info!($($arg)+);
        }
        else {
            std::println!($($arg)+);
        }
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => {{
        if log::logger().enabled(&$crate::logger::DUMMY) {
            log::warn!($($arg)+);
        }
        else {
            std::println!($($arg)+);
        }
    }};
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => {{
        if log::logger().enabled(&$crate::logger::DUMMY) {
            log::debug!($($arg)+);
        }
        else {
            std::println!($($arg)+);
        }
    }};
}

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => {{
        if log::logger().enabled(&$crate::logger::DUMMY) {
            log::trace!($($arg)+);
        }
        else {
            std::println!($($arg)+);
        }
    }};
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => {{
        if log::logger().enabled(&$crate::logger::DUMMY) {
            log::error!($($arg)+);
        }
        else {
            std::println!($($arg)+);
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::logger::*;
    use tempfile::tempdir;

    #[test]
    fn terminal_logger_init_test() {
        log_info!("INFO Test TO PRINTLN!");
        log_debug!("DEBUG Test TO PRINTLN!");

        if !log::logger().enabled(&crate::logger::DUMMY) {
            terminal_logger_init(LevelFilter::Debug);
        }

        log_info!("INFO Test TO LOGGER!");
        log_debug!("DEBUG Test TO LOGGER!");
    }

    #[test]
    fn combined_logger_init_test() {
        let log_dir = tempdir().expect("Failed to create temp directory!");

        log_info!("INFO Test TO PRINTLN!");
        log_debug!("DEBUG Test TO PRINTLN!");

        if !log::logger().enabled(&crate::logger::DUMMY) {
            combined_logger_init(
                LevelFilter::Debug,
                LevelFilter::Trace,
                log_dir.path().to_str().unwrap(),
                "test",
            );
        }

        log_info!("INFO Test TO combined LOGGER!");
        log_debug!("DEBUG Test TO combined LOGGER!");
    }

    #[test]
    fn results_macros_test() {
        log_info!("INFO Test TO PRINTLN!");
        results_info!("RESULTS Test from PRINTLN!");

        if !log::logger().enabled(&crate::logger::DUMMY) {
            terminal_logger_init(LevelFilter::Debug);
        }

        log_info!("INFO Test TO LOGGER!");
        results_info!("RESULTS Test from LOGGER!");

        log_info!("INFO Test TO LOGGER!");
        results_info!(info, "{}", "RESULTS Test from LOGGER2!");
        results_info!("{}", "RESULTS Test from LOGGER3!");
        let a = "results";
        results_info!(mode: a, "{}", "TEST");
        let b = "info";
        results_info!(mode: b, "{}", "TEST2");
    }
}
