// Copyright (c) 2025 Zensical and contributors

// SPDX-License-Identifier: MIT
// Third-party contributions licensed under DCO

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to
// deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.

// ----------------------------------------------------------------------------

//! Change.

use std::fmt::{self, Write};
use std::str::FromStr;

mod error;
mod kind;

pub use error::{Error, Result};
pub use kind::Kind;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Change.
#[derive(Debug)]
pub struct Change {
    /// Change kind.
    kind: Kind,
    /// Change description.
    description: String,
    /// Change is breaking.
    is_breaking: bool,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

#[allow(clippy::must_use_candidate)]
impl Change {
    /// Returns the change kind.
    #[inline]
    pub fn kind(&self) -> Kind {
        self.kind
    }

    /// Returns the change description.
    #[inline]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Returns whether the change is breaking.
    #[inline]
    pub fn is_breaking(&self) -> bool {
        self.is_breaking
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for Change {
    type Err = Error;

    /// Attempts to create a change from a string.
    ///
    /// # Errors
    ///
    /// This methods return [`Error::Format`][], if the string does not adhere
    /// to conventional commits format (without scopes), and [`Error::Kind`][],
    /// if the string does not correspond to a valid [`Kind`] variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use ctrl_z_changeset::Change;
    ///
    /// // Create change from string
    /// let change: Change = "fix: description".parse()?;
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(value: &str) -> Result<Self> {
        let Some((kind, description)) = value.split_once(": ") else {
            return Err(Error::Format);
        };

        // Check if we have a breaking change, denoted by an exclamation mark
        // at the end of the string, and extract and parse the change kind
        let (kind, is_breaking) = match kind.split_once('!') {
            Some((kind, _)) => (Kind::from_str(kind)?, true),
            None => (Kind::from_str(kind)?, false),
        };

        // Ensure description has no leading or trailing whitespace, as we need
        // to be as strict as possible with the format of commit messages
        if description != description.trim() {
            return Err(Error::Whitespace);
        }

        // Ensure description is not a sentence, i.e., doesn't end with a period
        if description.ends_with('.') {
            return Err(Error::Sentence);
        }

        // Ensure description is lowercase, unless it's an entire uppercase
        // word, e.g., an acronym like README, API, or URL
        if let Some(char) = description.chars().next() {
            if char.is_uppercase() {
                let word = description.split_whitespace().next().unwrap_or("");
                let is_acronym = word
                    .chars()
                    .all(|char| !char.is_alphabetic() || char.is_uppercase());

                // If not an acronym, return error
                if !is_acronym {
                    return Err(Error::Casing);
                }
            }
        }

        // Return change
        let description = description.to_string();
        Ok(Change { kind, description, is_breaking })
    }
}

// ----------------------------------------------------------------------------

impl fmt::Display for Change {
    /// Formats the change for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)?;
        if self.is_breaking {
            f.write_char('!')?;
        }

        // Write description
        f.write_str(": ")?;
        self.description.fmt(f)
    }
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    #[allow(clippy::bool_assert_comparison)]
    mod from_str {
        use std::str::FromStr;

        use crate::changeset::change::{Change, Error, Kind, Result};

        #[test]
        fn handles_non_breaking() -> Result {
            let change = Change::from_str("fix: description")?;
            assert_eq!(change.kind, Kind::Fix);
            assert_eq!(change.is_breaking, false);
            assert_eq!(change.description, "description");
            Ok(())
        }

        #[test]
        fn handles_breaking() -> Result {
            let change = Change::from_str("fix!: description")?;
            assert_eq!(change.kind, Kind::Fix);
            assert_eq!(change.is_breaking, true);
            assert_eq!(change.description, "description");
            Ok(())
        }

        #[test]
        fn errors_on_invalid_format() {
            for format in [
                "fix:description",
                "fix:  description",
                "fix :description",
                "fix description",
            ] {
                let res = Change::from_str(format);
                assert!(matches!(res, Err(Error::Format)));
            }
        }

        #[test]
        fn errors_on_invalid_kind() {
            for format in [
                " fix: description", // fmt
                "fix : description",
                "fxi: description",
            ] {
                let res = Change::from_str(format);
                assert!(matches!(res, Err(Error::Kind)));
            }
        }
    }
}
