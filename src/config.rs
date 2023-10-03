//! # Configuration utilities
//! A toolbox of small utilities to retrieve,write and compare INI-style configuration files.
//! Useful for initial configuration of binaries and to perform updates in existing installations.

use anyhow::Result;
use ini::Ini;
use std::{
    collections::HashSet,
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use crate::{log_debug, log_trace};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct IniCompare {
    pub updated: Vec<(IniParameter, IniParameter)>,
    pub added: Vec<IniParameter>,
    pub deleted: Vec<IniParameter>,
}

impl IniCompare {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct IniParameter {
    section: Option<String>,
    property: String,
    value: String,
}

/// Compare two ini files and return the differences against the first one
pub fn ini_compare(a: &PathBuf, b: &PathBuf) -> Result<IniCompare> {
    // Open File A and Extract Information
    let a_ini = Ini::load_from_file(a)?;

    // Open File B and Extract Information
    let b_ini = Ini::load_from_file(b)?;

    // Initialize empty results
    let mut results = IniCompare::new();

    // Find removed and added sections
    let a_sections: HashSet<&str> = a_ini.iter().filter_map(|(s, _)| s).collect();
    let b_sections: HashSet<&str> = b_ini.iter().filter_map(|(s, _)| s).collect();

    let deleted_sections = a_sections.difference(&b_sections);
    let added_sections = b_sections.difference(&a_sections);

    for &added_section in added_sections {
        if let Some((_, b_properties)) = b_ini
            .iter()
            .find(|&(b_section, _)| b_section == Some(added_section))
        {
            for b_property in b_properties.iter() {
                results.added.push(IniParameter {
                    section: Some(added_section.to_string()),
                    property: b_property.0.to_string(),
                    value: b_property.1.to_string(),
                })
            }
        }
    }

    for &deleted_section in deleted_sections {
        if let Some((_, a_properties)) = a_ini
            .iter()
            .find(|&(a_section, _)| a_section == Some(deleted_section))
        {
            for a_property in a_properties.iter() {
                results.deleted.push(IniParameter {
                    section: Some(deleted_section.to_string()),
                    property: a_property.0.to_string(),
                    value: a_property.1.to_string(),
                })
            }
        }
    }

    // Perform comparison section-by-section
    for (a_section, a_properties) in a_ini.iter() {
        if let Some((_, b_properties)) = b_ini.iter().find(|&(b_section, _)| a_section == b_section)
        {
            let a_keys: HashSet<&str> = a_properties.iter().map(|(key, _)| key).collect();
            let b_keys: HashSet<&str> = b_properties.iter().map(|(key, _)| key).collect();

            // Get additions
            let added_keys: HashSet<&str> =
                b_keys.difference(&a_keys).map(|x| x.to_owned()).collect();
            let mut added: Vec<IniParameter> = added_keys
                .iter()
                .filter_map(|k| {
                    b_properties.get(k).map(|v| IniParameter {
                        section: a_section.map(|x| x.to_string()),
                        property: k.to_string(),
                        value: v.to_string(),
                    })
                })
                .collect();

            // Get removals
            let removed_keys: HashSet<&str> =
                a_keys.difference(&b_keys).map(|x| x.to_owned()).collect();
            let mut removed: Vec<IniParameter> = removed_keys
                .iter()
                .filter_map(|k| {
                    a_properties.get(k).map(|v| IniParameter {
                        section: a_section.map(|x| x.to_string()),
                        property: k.to_string(),
                        value: v.to_string(),
                    })
                })
                .collect();

            // Get keys that remained
            let updated_keys: HashSet<&str> =
                a_keys.intersection(&b_keys).map(|x| x.to_owned()).collect();

            let mut updated: Vec<(IniParameter, IniParameter)> = vec![];

            for key in updated_keys.iter() {
                let a_value = a_properties.get(key);
                let b_value = b_properties.get(key);

                if a_value != b_value {
                    updated.push((
                        IniParameter {
                            section: a_section.map(|x| x.to_string()),
                            property: key.to_string(),
                            value: a_value.unwrap_or_default().to_string(),
                        },
                        IniParameter {
                            section: a_section.map(|x| x.to_string()),
                            property: key.to_string(),
                            value: b_value.unwrap_or_default().to_string(),
                        },
                    ));
                }
            }

            // Push current section results into global results
            results.added.append(&mut added);
            results.deleted.append(&mut removed);
            results.updated.append(&mut updated);
        }
    }

    Ok(results)
}

/// Update file using the comparison results
/// Currect Policy supports updates of specific fields
pub fn ini_update(new_ini_file: &PathBuf, comparison: IniCompare) -> Result<()> {
    let mut new_config = File::open(new_ini_file)?;

    // Convert file into a mutable stream so we can do "sed"-like operations over it
    let mut file_stream = String::new();
    new_config.read_to_string(&mut file_stream)?;

    // List of properties to keep the new values
    // EVERYTHING ELSE will be replaced with the previous protected value
    let unprotected_properties = ["config_file_version"];

    for updates in comparison.updated.iter() {
        if unprotected_properties
            .iter()
            .any(|&x| x == updates.1.property)
        {
            continue;
        }

        log_debug!(
            "[ini][update] SECTION: [{:?}] PROPERTY: [{:?}] VALUE: [{:?}] => [{:?}]",
            updates.1.section,
            updates.1.property,
            updates.1.value,
            updates.0.value,
        );

        file_stream = file_stream.replace(
            &format!("{} = {}", updates.1.property, updates.1.value),
            &format!("{} = {}", updates.0.property, updates.0.value),
        );
    }

    log_trace!("[ini][update] NEW CONFIGURATION:\n{}", file_stream);

    // Replace the file with new one
    let mut new_config = File::options().write(true).open(new_ini_file)?;

    new_config.write_all(file_stream.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use indoc::indoc;

    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn compare_test() {
        let mut a_ini = NamedTempFile::new().expect("Failed to create temp file!");

        let config = indoc! {r#"
        [version]
        ; format: <major>.<minor>. Example 1.2
        config_file_version = 10.0

        ; A list of compatible FPGA bitstreams.
        compatible_fpga = 2.0.0

        ; Possible values: A1, B2
        sys_variant = B2
        
        [log]
        ; This parameter sets the minimum log level that will be printed.
        ; 0 = Trace
        ; 1 = Debug
        ; 2 = Info
        ; 3 = Warning
        ; 4 = Error
        ; 5 = Fatal
        log_level = 0
        test_removed = 3

        [board_control]
        ref_clk_select = INT

        ; This setting deactivates dynamic fan speed control.
        always_apply_full_fan_speed = 0

        "#};

        writeln!(a_ini, "{}", config).expect("Failed to write to temp file!");

        let mut b_ini = NamedTempFile::new().expect("Failed to create temp file!");

        let config = indoc! {r#"
        [version]
        ; format: <major>.<minor>. Example 1.2
        config_file_version = 10.1

        ; A list of compatible FPGA bitstreams.
        ; If the bitstream loaded to the board is not found in the list below, the firmware will not operate.
        ; The version string should follow "X.Y.Z" format. All versions should be separated with a space character.
        ; Typically, this field should not be modified in a file provided in the release
        ; package. Note that entering a version that is not compatible might lead to firmware crash.
        compatible_fpga = 2.0.0

        ; Possible values: A1, B2
        ; Basing on this parameter value the FW configures the Tx Board and selects how many and which PA channels are to be used.
        ; The behavior when any other parameter is selected is undefined.
        sys_variant = UNDEFINED

        [log]
        ; This parameter sets the minimum log level that will be printed.
        ; 0 = Trace
        ; 1 = Debug
        ; 2 = Info
        ; 3 = Warning
        ; 4 = Error
        ; 5 = Fatal
        log_level = 2

        [power_control]
        test_added = YEAH

        [board_control]
        ref_clk_select = INT

        ; This setting deactivates dynamic fan speed control.
        always_apply_full_fan_speed = 0

"#};

        writeln!(b_ini, "{}", config).expect("Failed to write to temp file!");

        let mut results = ini_compare(
            &a_ini.into_temp_path().to_path_buf(),
            &b_ini.into_temp_path().to_path_buf(),
        )
        .unwrap();

        println!("{:#?}", results);

        let mut expected = IniCompare {
            updated: [
                (
                    IniParameter {
                        section: Some("version".to_string()),
                        property: "config_file_version".to_string(),
                        value: "10.0".to_string(),
                    },
                    IniParameter {
                        section: Some("version".to_string()),
                        property: "config_file_version".to_string(),
                        value: "10.1".to_string(),
                    },
                ),
                (
                    IniParameter {
                        section: Some("version".to_string()),
                        property: "sys_variant".to_string(),
                        value: "B2".to_string(),
                    },
                    IniParameter {
                        section: Some("version".to_string()),
                        property: "sys_variant".to_string(),
                        value: "UNDEFINED".to_string(),
                    },
                ),
                (
                    IniParameter {
                        section: Some("log".to_string()),
                        property: "log_level".to_string(),
                        value: "0".to_string(),
                    },
                    IniParameter {
                        section: Some("log".to_string()),
                        property: "log_level".to_string(),
                        value: "2".to_string(),
                    },
                ),
            ]
            .to_vec(),
            added: [IniParameter {
                section: Some("power_control".to_string()),
                property: "test_added".to_string(),
                value: "YEAH".to_string(),
            }]
            .to_vec(),
            deleted: [IniParameter {
                section: Some("log".to_string()),
                property: "test_removed".to_string(),
                value: "3".to_string(),
            }]
            .to_vec(),
        };

        // Sort the results to avoid issues with vector equality checks
        expected.added.sort_unstable();
        expected.deleted.sort_unstable();
        expected.updated.sort_unstable();

        // Sort the results to avoid issues with vector equality checks
        results.added.sort_unstable();
        results.deleted.sort_unstable();
        results.updated.sort_unstable();

        assert_eq!(expected, results);
    }
}
