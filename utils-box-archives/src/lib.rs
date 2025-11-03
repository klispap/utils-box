//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//! ## Archives
//! Extract files from Tar, Gz and Zip Files
//!
//! Mininal Example:
//! ```ignore
//! let archive: PathBuf = std::env::current_exe()
//!     .unwrap()
//!     .parent()
//!     .unwrap()
//!     .join("test_archive.tar.gz");
//!
//! let file: PathBuf = "treasure.hex".into();
//!
//! let destination: PathBuf = std::env::current_exe()
//!     .unwrap()
//!     .parent()
//!     .unwrap();
//!
//! archives::extract_file(archive, ArchiveType::Gz, file, destination).unwrap();
//!
//! ```
//!

pub mod archives;
