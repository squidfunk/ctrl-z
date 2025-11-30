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

use std::str::FromStr;

mod error;
mod kind;

pub use error::{Error, Result};
pub use kind::Kind;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    /// Change kind.
    pub kind: Kind,
    /// Change description.
    pub description: String,
    /// Change is breaking.
    pub is_breaking: bool,
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

        // Ensure description has no leading whitespace, as we chose to be as
        // strict as possible with the format of commit messages
        if description.chars().next().is_some_and(char::is_whitespace) {
            return Err(Error::Format);
        }

        // Return change
        let description = description.to_string();
        Ok(Change { kind, description, is_breaking })
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
                "fix:description", // fmt
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
