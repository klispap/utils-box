//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Bits
//! Convertions between different representations of raw bit streams
//!
//! Mininal Example:
//! ```ignore
//! let received_bit_stream: u64 = 0b110101000100111010110;
//!
//! let bytes = bits::bits_to_vec(received_bit_stream,21);
//!
//! println!("Received bit stream: {} ", bits::bit_vec_to_hex_string(&bytes));
//!
//! ```
//!

pub mod bits;
