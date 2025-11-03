//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Logger
//! Initialize terminal and file loggers fast. Macros for log printing to either log or stdout (if a global logger is not initialized)
//!
//! Mininal Example:
//! ```ignore
//!     log_info!("INFO Test TO PRINTLN!");
//!     log_debug!("DEBUG Test TO PRINTLN!");
//!
//!     terminal_logger_init(LevelFilter::Debug);
//!
//!     log_info!("INFO Test TO LOGGER!");
//!     log_debug!("DEBUG Test TO LOGGER!");
//!
//! ```
//!

#[macro_use]
pub mod logger;
