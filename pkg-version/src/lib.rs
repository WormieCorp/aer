// Copyright (c) 2021 Kim J. Nordmo and WormieCorp.
// Licensed under the MIT license. See LICENSE.txt file in the project

mod versions;

// use serde::Deserialize;
// use serde::Serialize;
use std::fmt::Display;

pub use semver::Version as SemVersion;
pub use versions::{chocolatey, FixVersion};

#[derive(Debug, PartialEq)]
//#[serde(untagged)]
pub enum Versions {
    SemVer(SemVersion),
    Choco(chocolatey::ChocoVersion),
}

impl Versions {
    pub fn to_choco(&self) -> chocolatey::ChocoVersion {
        match self {
            Versions::SemVer(semver) => chocolatey::ChocoVersion::from(semver.clone()),
            Versions::Choco(ver) => ver.clone(),
        }
    }

    pub fn to_semver(&self) -> SemVersion {
        match self {
            Versions::SemVer(semver) => semver.clone(),
            Versions::Choco(ver) => SemVersion::from(ver.clone()),
        }
    }
}

impl Display for Versions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Versions::SemVer(version) => version.fmt(f),
            Versions::Choco(version) => version.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_semver_should_create_semversion_from_choco_version() {
        let version =
            Versions::Choco(chocolatey::ChocoVersion::parse("2.1.0.5-alpha0055").unwrap());
        let expected = SemVersion::parse("2.1.0-alpha.55+5").unwrap();

        let actual = version.to_semver();

        assert_eq!(actual, expected);
    }

    #[test]
    fn to_semver_should_return_cloned_version_of_semver() {
        let version = Versions::SemVer(SemVersion::parse("5.2.2-alpha.5+55").unwrap());
        let expected = SemVersion::parse("5.2.2-alpha.5+55").unwrap();

        let actual = version.to_semver();

        assert_eq!(actual, expected);
    }

    #[test]
    fn to_choco_should_create_chocolatey_version_from_semver() {
        let version = Versions::SemVer(SemVersion::parse("1.0.5-beta.55+99").unwrap());
        let expected = chocolatey::ChocoVersion::parse("1.0.5-beta-0055").unwrap();

        let actual = version.to_choco();

        assert_eq!(actual, expected);
    }

    #[test]
    fn to_choco_should_returned_cloned_version_of_choco() {
        let version =
            Versions::Choco(chocolatey::ChocoVersion::parse("5.2.1.56-unstable-0050").unwrap());
        let expected = chocolatey::ChocoVersion::parse("5.2.1.56-unstable-0050").unwrap();

        let actual = version.to_choco();

        assert_eq!(actual, expected);
    }

    #[test]
    fn display_choco_version() {
        let version =
            Versions::Choco(chocolatey::ChocoVersion::parse("2.1.0-unstable-0050").unwrap());
        let expected = "2.1.0-unstable-0050";

        let actual = format!("{}", version);

        assert_eq!(actual, expected);
    }

    #[test]
    fn display_semver() {
        let version = Versions::SemVer(SemVersion::parse("2.5.2+build.50").unwrap());
        let expected = "2.5.2+build.50";

        let actual = format!("{}", version);

        assert_eq!(actual, expected);
    }
}
