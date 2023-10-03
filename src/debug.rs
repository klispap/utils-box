//! # Debug utilities
//! A toolbox of small utilities to debug data collections like vectors, maps, sets.
//! Useful for debug printing the contents of those collections in a meaningful way.

use core::fmt::{Binary, Display};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
    io,
    io::prelude::*,
};

use crate::{log_debug, log_info};

pub enum IdxMode {
    Based0,
    Based1,
}

pub fn vector_debug<T: Debug>(data: &[T], label: &str, idx_mode: IdxMode) {
    let idx_mode: usize = match idx_mode {
        IdxMode::Based0 => 0,
        IdxMode::Based1 => 1,
    };
    let idx_padding = (data.len() as f32).log(10.0).floor() as usize + 1;
    for (idx, value) in data.iter().enumerate() {
        log_debug!("{}[{:idx_padding$}] = {:?}", label, idx + idx_mode, value);
    }
}

pub fn vector_display<T: Display>(data: &[T], label: &str, idx_mode: IdxMode) {
    let idx_mode: usize = match idx_mode {
        IdxMode::Based0 => 0,
        IdxMode::Based1 => 1,
    };
    let idx_padding = (data.len() as f32).log(10.0).floor() as usize + 1;
    for (idx, value) in data.iter().enumerate() {
        log_info!("{}[{:idx_padding$}] = {}", label, idx + idx_mode, value);
    }
}

pub fn vector_display_bits<T: Binary>(data: &[T], label: &str, bits: usize, idx_mode: IdxMode) {
    let idx_mode: usize = match idx_mode {
        IdxMode::Based0 => 0,
        IdxMode::Based1 => 1,
    };
    let idx_padding = (data.len() as f32).log(10.0).floor() as usize + 1;
    for (idx, value) in data.iter().enumerate() {
        log_info!(
            "{}[{:idx_padding$}] = {:0bits$b}",
            label,
            idx + idx_mode,
            value
        );
    }
}

pub fn hashmap_display<K: Display + Ord + Hash, V: Display>(data: &HashMap<K, V>, label: &str) {
    let mut key_vec: Vec<&K> = data.keys().collect();
    key_vec.sort_unstable();

    for (idx, key) in key_vec.iter().enumerate() {
        let value = data.get(key).unwrap();
        log_info!("{}[{}]\t[{}] => {}", label, idx, key, value);
    }
}

pub fn hashmap_debug<K: Debug + Ord + Hash, V: Debug>(data: &HashMap<K, V>, label: &str) {
    let mut key_vec: Vec<&K> = data.keys().collect();
    key_vec.sort_unstable();

    for (idx, key) in key_vec.iter().enumerate() {
        log_debug!("{}[{}]\t[{:?}]", label, idx, key);
    }
}

pub fn hashset_display<K: Display + Ord + Hash, V: Display>(data: &HashSet<K>, label: &str) {
    let mut key_vec: Vec<&K> = data.iter().collect();
    key_vec.sort_unstable();

    for (idx, key) in key_vec.iter().enumerate() {
        log_info!("{}[{}]\t[{}]", label, idx, key);
    }
}

pub fn hashset_debug<K: Debug + Ord + Hash, V: Debug>(data: &HashSet<K>, label: &str) {
    let mut key_vec: Vec<&K> = data.iter().collect();
    key_vec.sort_unstable();

    for (idx, key) in key_vec.iter().enumerate() {
        let value = data.get(key).unwrap();
        log_debug!("{}[{}]\t[{:?}] => {:?}", label, idx, key, value);
    }
}

/// Use this function to momentarily pause execution during debugging
pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
