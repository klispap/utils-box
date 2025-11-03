//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Paths
//! Search paths for a specific file in directories with known or unknown paths
//!
//! Mininal Example:
//! ```ignore
//!     let paths = IncludePathsBuilder::new()
//!             .include_exe_dir()
//!             .include_known("/home/user/")
//!             .include_unknown("utils-box")
//!             .build();
//!
//!     let pattern = "test_*.tar";
//!
//!     let file_found_in = paths.search_glob(pattern);
//!
//! ```
//!

pub mod paths;
