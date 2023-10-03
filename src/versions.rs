//! # Versioning utilities
//! A toolbox of small utilities based on `semver.org`.
//! Useful for version control operations.

use anyhow::{bail, Result};
use regex::Regex;
use semver::Version;

use crate::log_trace;

/// Parse a provide slice and get a semver version in the form of <major>.<minor>.<patch>
/// If the input has only <major>.<minor>, we expand to <major>.<minor>.0
pub fn semver_parse(str: &str) -> Result<Version> {
    let full: Regex = Regex::new(r"^\d+\.\d+\.\d+$")?;
    let missing_patch: Regex = Regex::new(r"^\d+\.\d+$")?;
    let full_with_pre = Regex::new(r"^(\d+\.\d+\.\d+)\-(.*)")?;
    let missing_patch_with_pre = Regex::new(r"^(\d+\.\d+)\-(.*)")?;

    if full.is_match(str) {
        let cap = full.captures(str).unwrap();
        let result = cap[0].to_string();

        log_trace!("FULL: {}", result);

        if result.is_empty() {
            bail!(
                "[utils][semver_parse] Failed to capture X.X.X regex from [{}]!",
                str
            )
        } else {
            Ok(Version::parse(&result)?)
        }
    } else if missing_patch.is_match(str) {
        let cap = missing_patch.captures(str).unwrap();
        let result = cap[0].to_string();

        log_trace!("MISSING: {}", result);

        if result.is_empty() {
            bail!(
                "[utils][semver_parse] Failed to capture X.X regex from [{}]!",
                str
            )
        } else {
            return Ok(Version::parse(&format!("{}.0", result))?);
        };
    } else if full_with_pre.is_match(str) {
        let cap = full_with_pre.captures(str).unwrap();
        let result = cap[0].to_string();

        log_trace!("full_with_pre: {}", result);

        if result.is_empty() {
            bail!(
                "[utils][semver_parse] Failed to capture X.X.X-PRE regex from [{}]!",
                str
            )
        } else {
            return Ok(Version::parse(&result)?);
        };
    } else if missing_patch_with_pre.is_match(str) {
        let cap = missing_patch_with_pre.captures(str).unwrap();
        let result = cap[1].to_string();
        let pre = cap[2].to_string();

        log_trace!("missing_patch_with_pre: {}", result);

        if result.is_empty() {
            bail!(
                "[utils][semver_parse] Failed to capture X.X-PRE regex from [{}]!",
                str
            )
        } else {
            return Ok(Version::parse(&format!("{}.0-{}", result, pre))?);
        };
    } else {
        bail!(
            "[utils][semver_parse] Failed to capture any vesion regex from [{}]!",
            str
        )
    }
}

/// Parse a provide slice and get a semver version in the form of <major>.<minor>.<patch>
/// If the input has only <major>.<minor>, we expand to <major>.<minor>.0
/// In case of failure, return 0.0.0
pub fn semver_parse_or_default(str: &str) -> Version {
    semver_parse(str).unwrap_or(Version::new(0, 0, 0))
}

/// Extract version based on provided pattern capture and delimiter
/// Example Pattern: "btsys_intbrd_fw_v_(.*).hex"
/// Example delimiter: '_' to parse "btsys_intbrd_fw_v_0_9.hex" as version 0.9
pub fn semver_parse_regex(str: &str, pattern: &str, delimiter: &str) -> Result<Version> {
    let re: Regex = Regex::new(pattern)?;

    if re.is_match(str) {
        let cap = re.captures(str).unwrap();

        if cap.len() != 2 {
            bail!(
                "[utils][semver_parse_regex] Failed to capture any version group from [{}]!",
                str
            )
        }

        let result = cap[1].to_string();

        log_trace!("version capture from str: {}", result);

        let fix_delimiters = result.replace(delimiter, ".");

        semver_parse(&fix_delimiters)
    } else {
        bail!(
            "[utils][semver_parse_regex] Failed to capture any vesion regex from [{}]!",
            str
        )
    }
}

#[cfg(test)]
mod tests {
    use semver::{Prerelease, Version};

    use super::{semver_parse, semver_parse_regex};

    #[test]
    fn semver_parse_test() {
        let version = "0.9.0";
        assert_eq!(semver_parse(version).unwrap(), Version::new(0, 9, 0));

        let version = "0.9";
        assert_eq!(semver_parse(version).unwrap(), Version::new(0, 9, 0));

        let version = "0.9.2-1e341234";
        let mut expected = Version::new(0, 9, 2);
        expected.pre = Prerelease::new("1e341234").unwrap();
        assert_eq!(semver_parse(version).unwrap(), expected);

        let version = "0.9-1-e341234";
        let mut expected = Version::new(0, 9, 0);
        expected.pre = Prerelease::new("1-e341234").unwrap();
        assert_eq!(semver_parse(version).unwrap(), expected);

        let version = "0.9-2";
        let mut expected = Version::new(0, 9, 0);
        expected.pre = Prerelease::new("2").unwrap();
        assert_eq!(semver_parse(version).unwrap(), expected);
    }

    #[test]
    fn semver_parse_regex_test() {
        let input = "btsys_intbrd_boot_config_v_0_9.hex";

        let pattern = "btsys_intbrd_boot_config_v_(.*).hex";

        let expected_version = Version::new(0, 9, 0);

        assert_eq!(
            semver_parse_regex(&input, &pattern, "_").unwrap(),
            expected_version
        );
    }
}
