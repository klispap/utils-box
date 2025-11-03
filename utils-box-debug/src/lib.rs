//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Debug
//! Print in log or stdout debug information from vectors, hashmaps in a human readable way.
//! Pause execution at specific moments to make debugging easier.
//!
//! Mininal Example:
//! ```ignore
//!     
//!     // Complex data operations before [..]
//!
//!     let data: Vec<f64> = (0..100).iter().map(|&x| x * f64::PI).collect();
//!
//!     // Print debug information from data vector
//!     vector_display(&data[0..10],"Mult_PI", IdxMode::Based1);
//!     // Pause execution to check values
//!     pause();
//!
//!     // Complex data operations after [..]
//!
//! ```
//!

#[cfg(not(tarpaulin_include))]
pub mod debug;
