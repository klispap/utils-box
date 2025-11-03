//! # Path utilities
//! A toolbox of small utilities to handle paths for files.
//! Useful for searching for files in a pre-determined list of directories or git repositories.

use anyhow::{Result, anyhow, bail};
use directories::BaseDirs;
use glob::{MatchOptions, glob_with};
use names::{Generator, Name};
use std::{env, path::PathBuf, str::FromStr};
use walkdir::{DirEntry, WalkDir};

use utils_box_logger::log_debug;

#[derive(Debug, Clone)]
pub struct IncludePaths {
    known_paths: Vec<PathBuf>,
    unknown_paths: Vec<PathBuf>,
}

pub struct IncludePathsBuilder {
    paths: IncludePaths,
}

impl Default for IncludePathsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IncludePathsBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            paths: IncludePaths {
                known_paths: vec![],
                unknown_paths: vec![],
            },
        }
    }

    /// Add another include directory that we know the path to seek into
    /// The directories are considered to be **absolute** if starting with `/` or relative to current folder otherwise
    /// You can chain multiple calls
    pub fn include_known(&mut self, path: &str) -> &mut Self {
        let new = self;
        new.paths.known_paths.push(PathBuf::from_str(path).unwrap());
        new
    }

    /// Add current executable directory to seek into
    /// You can chain multiple calls
    pub fn include_exe_dir(&mut self) -> &mut Self {
        let new = self;
        new.paths
            .known_paths
            .push(std::env::current_exe().unwrap().parent().unwrap().into());
        new
    }

    /// Add another include directory that we do not know the path to, to seek into
    /// We will start seeking the directory first by backtracing from the  the current folder until we reach the $HOME directory.
    /// Once we find the directory, then we will check inside for the files.
    /// Useful for testing inside project data.
    /// For example `cargo test` will execute from the `./target` folder and you have your test data inside a `config` folder somewhere in your project.
    /// The same fodler will be deployed in a subfolder next to the binary in deployment.
    /// You can use this function to include the `config` folder as unknown and the code will discover the directory by itself.
    /// This feature is useful to avoid having multiple includes with `../..` etc to cover all scenarios.
    /// You can chain multiple calls
    pub fn include_unknown(&mut self, path: &str) -> &mut Self {
        let new = self;
        new.paths
            .unknown_paths
            .push(PathBuf::from_str(path).unwrap());
        new
    }

    pub fn build(&mut self) -> IncludePaths {
        self.paths.clone()
    }
}

impl IncludePaths {
    /// Seek File in include directories first
    /// If not found, fall-back in unknown path search (if applicable)
    pub fn seek(&self, file: &str) -> Result<PathBuf> {
        let seek_dir = self.seek_in_known(file);

        if seek_dir.is_ok() {
            return seek_dir;
        }

        self.seek_in_unknown(file).map_err(|_| {
            anyhow!("[include_paths][seek] Failed to find file in directories in included known & unknown paths")
        })
    }

    /// Seek only in included known paths (ingoring included unknown paths)
    pub fn seek_in_known(&self, file: &str) -> Result<PathBuf> {
        let file = PathBuf::from_str(file)?;
        self.known_paths
            .iter()
            .find_map(|f| {
                if f.join(&file).exists() {
                    Some(f.join(&file))
                } else {
                    None
                }
            })
            .ok_or(anyhow!(
                "[include_paths][seek_in_known] Failed to find file in directories in included known paths"
            ))
    }

    /// Seek only using the directory names stored with their paths as unknown
    /// Tries to locate the repositories in the current working directory and then going backwards until reaching `$HOME` or `%HOME%`
    pub fn seek_in_unknown(&self, file: &str) -> Result<PathBuf> {
        // Get HOME directory
        let _home_dir: PathBuf = if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.home_dir().to_path_buf()
        } else {
            bail!(
                "[include_paths][seek_in_unknown] Failed to retrieve system's home directory path"
            )
        };

        // Get current working dir
        let current_dir: PathBuf = if let Ok(curr) = env::current_dir() {
            curr.to_path_buf().canonicalize()?
        } else {
            bail!("[include_paths][seek_in_unknown] Failed to retrieve current directory path")
        };

        for repo in self.unknown_paths.iter().rev() {
            for parent_dir in current_dir.ancestors() {
                for entry in WalkDir::new(parent_dir)
                    .follow_links(true)
                    .into_iter()
                    .filter_entry(|e| {
                        (e.file_type().is_dir() || e.path_is_symlink()) && is_not_hidden(e)
                    })
                {
                    let e_path = match entry {
                        Err(_) => continue,
                        Ok(ref e) => e.path(),
                    };

                    match e_path.file_name() {
                        Some(f_name) => {
                            if f_name == repo && e_path.join(file).exists() {
                                return Ok(e_path.join(file));
                            } else {
                                continue;
                            }
                        }
                        None => continue,
                    }
                }
            }
        }

        bail!(
            "[include_paths][seek_in_unknown] Failed to find file in directories in included unknown paths"
        );
    }

    /// Search in directories in known paths first for files matching the glob pattern requested
    /// If not found, fall-back in unknown paths search (if applicable)
    pub fn search_glob(&self, pattern: &str) -> Vec<PathBuf> {
        let seek_dir = self.search_glob_known(pattern);

        if !seek_dir.is_empty() {
            return seek_dir;
        }

        self.search_glob_unknown(pattern)
    }

    /// Search in included directories for files matching the glob pattern requested
    pub fn search_glob_known(&self, pattern: &str) -> Vec<PathBuf> {
        let options = MatchOptions {
            case_sensitive: true,
            // Allow relative paths
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        let detected_files: Vec<PathBuf> = self
            .known_paths
            .iter()
            .flat_map(|dir| {
                let new_pattern = dir.join("**/").join(pattern);
                let new_pattern = new_pattern.to_str().unwrap();
                glob_with(new_pattern, options)
                    .unwrap()
                    .filter_map(|path| path.ok())
                    .collect::<Vec<PathBuf>>()
            })
            .collect();

        log_debug!(
            "[include_paths][search_glob_known] Detected files for [{}]: \n{:?}",
            pattern,
            detected_files
        );

        detected_files
    }

    /// Search in included repsotories for files matching the glob pattern requested
    pub fn search_glob_unknown(&self, pattern: &str) -> Vec<PathBuf> {
        let options = MatchOptions {
            case_sensitive: true,
            // Allow relative paths
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        // Get HOME directory
        let home_dir: PathBuf = if let Some(base_dirs) = BaseDirs::new() {
            base_dirs.home_dir().to_path_buf()
        } else {
            log_debug!(
                "[include_paths][search_glob_unknown] Failed to retrieve system's home directory path"
            );
            PathBuf::new()
        };

        let detected_files: Vec<PathBuf> = {
            let mut tmp: Vec<PathBuf> = vec![];
            for repo in self.unknown_paths.iter() {
                let mut dir: Vec<PathBuf> = WalkDir::new(&home_dir)
                    .follow_links(true)
                    .into_iter()
                    .filter_entry(|e| e.file_type().is_dir() && is_not_hidden(e))
                    .filter_map(|entry| {
                        let dir = match entry {
                            Err(_) => return None,
                            Ok(ref e) => e.path(),
                        };

                        match dir.file_name() {
                            Some(f_name) => {
                                if f_name != repo {
                                    return None;
                                }
                            }
                            None => return None,
                        }

                        let new_pattern = dir.join("**/").join(pattern);
                        let new_pattern = new_pattern.to_str().unwrap();

                        Some(
                            glob_with(new_pattern, options)
                                .unwrap()
                                .filter_map(|path| path.ok())
                                .collect::<Vec<PathBuf>>(),
                        )
                    })
                    .flatten()
                    .collect::<Vec<PathBuf>>();

                tmp.append(&mut dir);
            }

            tmp
        };

        log_debug!(
            "[include_paths][search_glob_unknown] Detected files for [{}]: \n{:?}",
            pattern,
            detected_files
        );

        detected_files
    }
}

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with('.'))
        .unwrap_or(false)
}

/// Generate random filename
pub fn random_filename() -> String {
    let mut generator = Generator::with_naming(Name::Numbered);
    generator.next().unwrap()
}

/// Generate timestamp for filenames
pub fn timestamp_filename() -> String {
    format!("{}", chrono::offset::Utc::now().format("%Y%m%d_%H%M%S"))
}

#[cfg(test)]
mod tests {
    use crate::paths::*;

    #[test]
    fn include_path_test() {
        let paths = IncludePathsBuilder::new()
            .include_known("/test/")
            .include_known("/2/")
            .include_exe_dir()
            .include_known("test_data/")
            .build();

        println!("Paths: {:?}", paths);

        let file = "test_archives.tar";
        assert_eq!(
            std::fs::canonicalize(PathBuf::from_str("test_data/test_archives.tar").unwrap())
                .unwrap(),
            std::fs::canonicalize(paths.seek(file).unwrap()).unwrap()
        );
    }

    #[test]
    fn include_repos_test() {
        let paths = IncludePathsBuilder::new()
            .include_unknown("utils-box")
            .build();

        println!("Paths: {:?}", paths);

        let file = "src/paths.rs";
        assert_eq!(
            std::fs::canonicalize(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("../../../src/paths.rs")
            )
            .unwrap(),
            std::fs::canonicalize(paths.seek_in_unknown(file).unwrap()).unwrap()
        );
    }

    #[test]
    fn include_seek_all_test() {
        let paths = IncludePathsBuilder::new()
            .include_known("/test/")
            .include_known("/2/")
            .include_exe_dir()
            .include_known("tests_data/")
            .include_unknown("utils-box")
            .build();

        println!("Paths: {:?}", paths);

        let file = "src/paths.rs";
        assert_eq!(
            std::fs::canonicalize(
                std::env::current_exe()
                    .unwrap()
                    .parent()
                    .unwrap()
                    .join("../../../src/paths.rs")
            )
            .unwrap(),
            std::fs::canonicalize(paths.seek(file).unwrap()).unwrap()
        );
    }

    #[test]
    fn search_glob_dir_test() {
        let paths = IncludePathsBuilder::new()
            .include_known("test_data/")
            .build();

        println!("Paths: {:?}", paths);

        let pattern = "test_*.tar";

        let results = paths.search_glob_known(pattern);

        assert!(!results.is_empty());
    }

    #[test]
    fn search_glob_repos_test() {
        let paths = IncludePathsBuilder::new()
            .include_unknown("utils-box")
            .build();

        println!("Paths: {:?}", paths);

        let pattern = "test_*.tar";

        let results = paths.search_glob_unknown(pattern);

        assert!(!results.is_empty());
    }

    #[test]
    fn search_glob_all_test() {
        let paths = IncludePathsBuilder::new()
            .include_exe_dir()
            .include_unknown("utils-box")
            .build();

        println!("Paths: {:?}", paths);

        let pattern = "test_*.tar";

        let results = paths.search_glob(pattern);

        assert!(!results.is_empty());
    }
}
