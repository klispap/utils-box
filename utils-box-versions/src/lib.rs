//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Versions
//! version parser from strings using the `semver.org` notations
//!
//! Mininal Example:
//! ```ignore
//!    let version = "0.9.2-1e341234";
//!
//!     let mut expected = Version::new(0, 9, 2);
//!     expected.pre = Prerelease::new("1e341234").unwrap();
//!
//!     assert_eq!(semver_parse(version).unwrap(), expected);
//!
//! ```
//!

pub mod versions;
