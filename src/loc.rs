//! The `LocDiff` struct and related utilities.
//!
//! This module defines the `LocDiff` data structure together with its utility
//! enum `LocParseError`.
//!
//! LOC is the abbreviation for the number of **l**ines **o**f **c**ode a
//! project has and a possible measure to tell how this project has grown or
//! shrunk due to a set of changes applied to it.

use std::num::ParseIntError;

/// The LOC diff a certain commit introduces.
#[derive(Debug)]
pub struct LocDiff {
    /// The count of insertions.
    added: Option<u32>,

    /// The count of deletions.
    removed: Option<u32>,

    /// The affected file.
    file: String,
}

/// The methods and associated functions on and for a `LocDiff` instance,
/// respectively.
impl LocDiff {
    /// The getter method for the field `file` of the corresponding struct.
    ///
    /// Since the struct `LocDiff` defines its field `file` to be private, this
    /// getter is utilised in order to operate on this field.  It returns a
    /// read-only deep copy of the content.
    pub fn file(&self) -> String {
        String::from(&self.file)
    }

    /// Calculate the LOC diff.
    pub fn loc(&self) -> i64 {
        if self.added.is_none() && self.removed.is_none() {
            0
        } else {
            self.added.unwrap() as i64 - self.removed.unwrap() as i64
        }
    }

    /// Extract the LOC diff information from the given line.
    pub fn parse(loc: &str) -> Result<Self, LocParseError> {
        let (added, remainder) = loc
            .split_once('\t')
            .ok_or(LocParseError::FirstTabulatorMissing)?;
        let (removed, file) = remainder
            .split_once('\t')
            .ok_or(LocParseError::SecondTabulatorMissing)?;

        let added = if added == "-" {
            None
        } else {
            Some(added.parse().map_err(LocParseError::AddedParseError)?)
        };

        let removed = if removed == "-" {
            None
        } else {
            Some(removed.parse().map_err(LocParseError::RemovedParseError)?)
        };

        Ok(Self {
            added,
            removed,
            file: String::from(file),
        })
    }
}

/// The set of errors which may occur.
///
/// This enum describes the possible errors when parsing the LOC diff
/// information.  A valid LOC diff consists of
/// * the integral number of insertions,
/// * the integral number of deletions, and
/// * the affected file
/// with each of these pieces of information being separated by a tab character.
///
/// In case that some of these assumptions should fail, an according error from
/// this enum will occur.
#[derive(Debug)]
pub enum LocParseError {
    /// The tab character between the insertions and deletions is missing.
    FirstTabulatorMissing,

    /// The tab character between the deletions and the file's name is missing.
    SecondTabulatorMissing,

    /// The number of insertions could not be parsed correctly.
    AddedParseError(ParseIntError),

    /// The number of deletions could not be parsed correctly.
    RemovedParseError(ParseIntError),
}
