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

//! Version increment.

use std::fmt;

use crate::changeset::change::Kind;
use crate::changeset::Change;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Version increment.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Increment {
    /// Patch increment.
    Patch,
    /// Minor increment.
    Minor,
    /// Major increment.
    Major,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Change {
    /// Returns the corresponding version increment, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use ctrl_z_changeset::{Change, Increment};
    ///
    /// // Create increment from change
    /// let change: Change = "fix: description".parse()?;
    /// assert_eq!(change.as_increment(), Some(Increment::Patch));
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn as_increment(&self) -> Option<Increment> {
        let increment = match self.kind() {
            Kind::Feature => Increment::Minor,
            Kind::Fix => Increment::Patch,
            Kind::Performance => Increment::Patch,
            Kind::Refactor => Increment::Patch,
            _ => return None,
        };

        // If a version increment is determined, check for breaking changes,
        // as they must always lead to a major version increment
        if self.is_breaking() {
            Some(Increment::Major)
        } else {
            Some(increment)
        }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl fmt::Display for Increment {
    /// Formats the increment for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Increment::Patch => f.write_str("patch"),
            Increment::Minor => f.write_str("minor"),
            Increment::Major => f.write_str("major"),
        }
    }
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    mod as_increment {
        use std::str::FromStr;

        use crate::changeset::change::{Change, Result};
        use crate::changeset::version::Increment;

        #[test]
        fn handles_non_breaking() -> Result {
            let change = Change::from_str("fix: description")?;
            assert_eq!(change.as_increment(), Some(Increment::Patch));
            Ok(())
        }

        #[test]
        fn handles_breaking() -> Result {
            let change = Change::from_str("fix!: description")?;
            assert_eq!(change.as_increment(), Some(Increment::Major));
            Ok(())
        }
    }
}
