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

//! Change kind.

use std::str::FromStr;

use super::error::{Error, Result};

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Change kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Bugfix.
    Fix,
    /// Feature.
    Feature,
    /// Performance improvement.
    Performance,
    /// Refactor.
    Refactor,
    /// Documentation.
    Docs,
    /// Test.
    Test,
    /// Chore.
    Chore,
    /// Build.
    Build,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for Kind {
    type Err = Error;

    /// Attempts to create a change kind from a string.
    ///
    /// # Errors
    ///
    /// This methods return [`Error::Kind`], if the string does not correspond
    /// to a valid [`Kind`] variant.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use ctrl_z_changeset::change::Kind;
    ///
    /// // Create change kind from string
    /// let kind: Kind = "fix".parse()?;
    /// # Ok(())
    /// # }
    /// ```
    fn from_str(value: &str) -> Result<Self> {
        match value {
            "fix" => Ok(Kind::Fix),
            "feature" => Ok(Kind::Feature),
            "performance" => Ok(Kind::Performance),
            "refactor" => Ok(Kind::Refactor),
            "docs" => Ok(Kind::Docs),
            "test" => Ok(Kind::Test),
            "chore" => Ok(Kind::Chore),
            "build" => Ok(Kind::Build),
            _ => Err(Error::Kind),
        }
    }
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    mod from_str {
        use std::str::FromStr;

        use crate::changeset::change::{Error, Kind, Result};

        #[test]
        fn handles_valid_variants() -> Result {
            for (value, kind) in [
                ("fix", Kind::Fix),
                ("feature", Kind::Feature),
                ("performance", Kind::Performance),
                ("refactor", Kind::Refactor),
                ("docs", Kind::Docs),
                ("test", Kind::Test),
                ("chore", Kind::Chore),
                ("build", Kind::Build),
            ] {
                assert_eq!(Kind::from_str(value)?, kind);
            }
            Ok(())
        }

        #[test]
        fn errors_on_invalid_variant() {
            for value in ["fi x", "feat", "perf", "doc", "testing"] {
                let res = Kind::from_str(value);
                assert!(matches!(res, Err(Error::Kind)));
            }
        }

        #[test]
        fn errors_on_invalid_casing() {
            for value in ["Fix", "FEATURE"] {
                let res = Kind::from_str(value);
                assert!(matches!(res, Err(Error::Kind)));
            }
        }
    }
}
