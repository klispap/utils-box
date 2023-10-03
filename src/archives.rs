//! # Archives utilities
//! A toolbox of small utilities that extract files from archives.
//! Useful for retrieving files from various types of archives like tar, tar.gz, zip.

use anyhow::{bail, Result};
use flate2::read::GzDecoder;
use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};
use tar::Archive;
use zip::ZipArchive;

use crate::{log_info, log_trace};

pub static GZ_SIGNATURE: [u8; 3] = [0x1F, 0x8B, 0x08];
pub static ZIP_SIGNATURE: [u8; 3] = [0x50, 0x4B, 0x03];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveType {
    Tar,
    Gz,
    Zip,
}

/// Entry Point:
/// Extract the selected file from the provided archive into the selected destination directory
/// It detects if it is a compressed archive using the magic numbers from: https://www.garykessler.net/library/file_sigs.html
pub fn archive_extract_file(
    archive: PathBuf,
    filename: PathBuf,
    destination: PathBuf,
) -> Result<()> {
    // Check archive type
    let mut file = File::open(&archive)?;
    let mut magic_number: Vec<u8> = vec![0x0; 3];

    file.read_exact(&mut magic_number)?;

    if magic_number == GZ_SIGNATURE {
        log_info!("[archive_extract_file] Detected [tar.gz] archive");
        extract_file(archive, ArchiveType::Gz, filename, destination)
    } else if magic_number == ZIP_SIGNATURE {
        log_info!("[archive_extract_all] Detected [zip] archive");
        extract_file(archive, ArchiveType::Zip, filename, destination)
    } else {
        log_info!("[archive_extract_file] Detected [tar] archive");
        extract_file(archive, ArchiveType::Tar, filename, destination)
    }
}

/// Entry Point:
/// Extract all contents from the provided archive into the selected destination directory
/// The destination directory will be created if not already available
/// It detects if it is a compressed archive using the magic numbers from: https://www.garykessler.net/library/file_sigs.html
pub fn archive_extract_all(archive: PathBuf, destination: PathBuf) -> Result<()> {
    // Check archive type
    let mut file = File::open(&archive)?;
    let mut magic_number: Vec<u8> = vec![0x0; 3];

    file.read_exact(&mut magic_number)?;

    if magic_number == GZ_SIGNATURE {
        log_info!("[archive_extract_all] Detected [tar.gz] archive");
        extract_all(archive, ArchiveType::Gz, destination)
    } else if magic_number == ZIP_SIGNATURE {
        log_info!("[archive_extract_all] Detected [zip] archive");
        extract_all(archive, ArchiveType::Zip, destination)
    } else {
        log_info!("[archive_extract_all] Detected [tar.gz] archive");
        extract_all(archive, ArchiveType::Tar, destination)
    }
}

/// Extract the selected file from the provided archive into the selected destination directory
fn extract_file(
    archive: PathBuf,
    archive_type: ArchiveType,
    filename: PathBuf,
    destination: PathBuf,
) -> Result<()> {
    match archive_type {
        ArchiveType::Tar => {
            let mut ar = Archive::new(File::open(archive)?);

            // Check the contents for the requested file
            for archived_file in ar.entries()? {
                // Unwrap the file
                let mut ar_file = archived_file?;

                // Check if it is the file we need
                if ar_file.path()? == filename {
                    let _ = std::fs::create_dir(destination.clone());
                    ar_file.unpack_in(&destination)?;

                    return Ok(());
                }
            }

            bail!("[extract_file][tar] Failed to find requested file!");
        }
        ArchiveType::Gz => {
            let file = File::open(archive)?;
            let decompressed = GzDecoder::new(file);
            let mut ar = Archive::new(decompressed);

            // Check the contents for the requested file
            for archived_file in ar.entries()? {
                // Unwrap the file
                let mut ar_file = archived_file?;

                // Check if it is the file we need
                if ar_file.path()? == filename {
                    let _ = std::fs::create_dir(destination.clone());
                    ar_file.unpack_in(&destination)?;

                    return Ok(());
                }
            }

            bail!("[extract_file][gz] Failed to find requested file!");
        }
        ArchiveType::Zip => {
            let file = File::open(archive)?;
            let mut ar = ZipArchive::new(file)?;

            for i in 0..ar.len() {
                let mut in_file = ar.by_index(i)?;
                let outpath = match in_file.enclosed_name() {
                    Some(path) => path,
                    None => {
                        log_trace!("Entry {} has a suspicious path", in_file.name());
                        continue;
                    }
                };

                if outpath.file_name() == Some(filename.as_os_str()) {
                    let _ = std::fs::create_dir(destination.clone());
                    let mut outfile = File::create(destination.join(filename))?;
                    io::copy(&mut in_file, &mut outfile)?;

                    return Ok(());
                }
            }

            bail!("[extract_file][zip] Failed to find requested file !");
        }
    };
}

/// Extract all contents from the provided archive into the selected destination directory
/// The destination directory will be created if not already available
fn extract_all(archive: PathBuf, archive_type: ArchiveType, destination: PathBuf) -> Result<()> {
    match archive_type {
        ArchiveType::Tar => {
            let mut ar = Archive::new(File::open(archive)?);
            let _ = std::fs::create_dir(destination.clone());
            ar.unpack(destination)?;
        }
        ArchiveType::Gz => {
            let file = File::open(archive)?;
            let decompressed = GzDecoder::new(file);
            let mut ar = Archive::new(decompressed);
            let _ = std::fs::create_dir(destination.clone());
            ar.unpack(destination)?;
        }
        ArchiveType::Zip => {
            let file = File::open(archive)?;
            let mut ar = ZipArchive::new(file)?;
            let _ = std::fs::create_dir(destination.clone());
            ar.extract(destination)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::archives::*;
    use named_lock::*;

    #[test]
    fn extract_tar_test() {
        let archive: PathBuf = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("../../../test_data/test_archives.tar");

        let file: PathBuf = "happy_cloud.jpg".into();

        // Create a named lock to make sure no other test messes with the same files!
        let lock = NamedLock::create("archives_tests").unwrap();
        let _guard = lock.lock().unwrap();

        let destination: PathBuf = std::env::temp_dir().join("utils-box");

        let _ = std::fs::remove_dir_all(destination.clone());

        extract_file(archive, ArchiveType::Tar, file, destination).unwrap();
    }

    #[test]
    fn extract_tar_all_test() {
        let archive: PathBuf = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("../../../test_data/test_archives.tar");

        // Create a named lock to make sure no other test messes with the same files!
        let lock = NamedLock::create("archives_tests").unwrap();
        let _guard = lock.lock().unwrap();

        let destination: PathBuf = std::env::temp_dir().join("utils-box");

        let _ = std::fs::remove_dir_all(destination.clone());

        extract_all(archive, ArchiveType::Tar, destination).unwrap();
    }

    #[test]
    fn extract_zip_test() {
        let archive: PathBuf = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("../../../test_data/test_archives.zip");

        let file: PathBuf = "lorem.txt".into();

        let destination: PathBuf = std::env::temp_dir().join("utils-box");

        // Create a named lock to make sure no other test messes with the same files!
        let lock = NamedLock::create("archives_tests").unwrap();
        let _guard = lock.lock().unwrap();

        let _ = std::fs::remove_dir_all(destination.clone());

        extract_file(archive, ArchiveType::Zip, file, destination).unwrap();
    }

    #[test]
    fn extract_zip_all_test() {
        let archive: PathBuf = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("../../../test_data/test_archives.zip");

        let destination: PathBuf = std::env::temp_dir().join("utils-box");

        // Create a named lock to make sure no other test messes with the same files!
        let lock = NamedLock::create("archives_tests").unwrap();
        let _guard = lock.lock().unwrap();

        let _ = std::fs::remove_dir_all(destination.clone());

        extract_all(archive, ArchiveType::Zip, destination).unwrap();
    }
}
