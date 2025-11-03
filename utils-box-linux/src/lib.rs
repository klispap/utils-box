//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Linux OS utilities
//! A toolbox of small utilities for Linux environments.
//! Useful for fast binary development via the provided signal handling and panic hooks.
//!
//!

#[cfg(target_family = "unix")]
#[cfg(not(tarpaulin_include))]
pub mod threads;
