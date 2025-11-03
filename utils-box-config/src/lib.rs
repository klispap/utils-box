//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Mathematics
//! A collection of useful methematic methods used in various DSP and other applications
//!
//! ## Config
//! Manipulate INI-style configuration files by checking for changes, updates etc
//!
//! Mininal Example:
//! ```ignore
//!     let mut config_changes = ini_compare(
//!         &old_config_path.to_path_buf(),
//!         &new_config_path.to_path_buf(),
//!     )
//!     .unwrap();
//!
//!    println!("{:#?}", config_changes);
//!
//! ```
//!

pub mod config;
