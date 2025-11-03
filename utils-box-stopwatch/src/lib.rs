//! # Summary
//! A toolbox library that holds a useful collection of small unitilies written in Rust that make our life easier when writting Rust applications.
//!
//! # Utilities provided:
//!
//! ## Mathematics
//! A collection of useful methematic methods used in various DSP and other applications
//!
//! ## Stopwatch and Timekeper
//! Keep track of execution times in various points in binaries. Print records.
//!
//! Minimal Example:
//! ```ignore
//!    let mut s = TimeKeeper::init();
//!    let mut t = TimeKeeper::init();
//!
//!    s.totals();
//!
//!    s.lap("init");
//!
//!    for _ in 0..5 {
//!        std::thread::sleep(Duration::from_millis(5));
//!        s.lap("loop");
//!        t.lap("loop");
//!    }
//!    s.lap_totals("loop");
//!    std::thread::sleep(Duration::from_millis(1234));
//!    s.lap("after");
//!
//!    s.totals();
//!    t.totals();
//!
//!    s.merge(t);
//!
//!    s.totals();
//!
//! ```
//!

pub mod stopwatch;
