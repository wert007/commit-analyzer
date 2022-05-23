//! The `Loc` struct and related utilities.
//!
//! This module defines the `Loc` data structure together with its utility enum
//! `LocParseError`.

use std::num::ParseIntError;

#[derive(Debug)]
pub struct Loc {
    added: Option<u32>,
    removed: Option<u32>,
    file: String,
}

impl Loc {
    /// The getter method for the field `file` of the corresponding struct.
    ///
    /// Since the struct `Loc` defines its field `file` to be private, this
    /// getter is utilised in order to operate on this field.  It returns a
    /// read-only deep copy of the content.
    pub fn file(&self) -> String {
        String::from(&self.file)
    }

    pub fn loc(&self) -> i64 {
        if self.added.is_none() && self.removed.is_none() {
            0
        } else {
            self.added.unwrap() as i64 - self.removed.unwrap() as i64
        }
    }

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
        let file = file.into();
        Ok(Self {
            added,
            removed,
            file,
        })
    }
}

#[derive(Debug)]
pub enum LocParseError {
    FirstTabulatorMissing,
    SecondTabulatorMissing,
    AddedParseError(ParseIntError),
    RemovedParseError(ParseIntError),
}
