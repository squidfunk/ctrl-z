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

//! Version set.

use semver::Version;
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::fmt;
use std::ops::{Bound, RangeBounds};
use std::str::FromStr;

use super::error::Result;
use super::Repository;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Version set.
///
/// This data type represents a set of versions in a given repository. Versions
/// are ordered from new to old, so iteration and range queries are simple.
pub struct Versions {
    /// Tagged versions.
    tags: BTreeSet<Reverse<Version>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    /// Returns the version set of the repository.
    ///
    /// This method only extracts the tags matching semantic version specifiers
    /// from the given repository, and returns a version set. Tags must abide
    /// to the `vMAJOR.MINOR.PATCH` format, but can include pre-release and
    /// build suffixes as well. Tags are parsed with [`Version::from_str`].
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`][] if the operation fails.
    ///
    /// [`Error::Git`]: crate::repository::Error::Git
    pub fn versions(&self) -> Result<Versions> {
        let tags = self.inner.tag_names(Some("v[0-9]*.[0-9]*.[0-9]**"))?;
        let iter = tags.iter().flatten().map(|name| {
            Ok(Reverse(Version::from_str(name.trim_start_matches('v'))?))
        });

        // Collect and return version set
        let tags = iter.collect::<Result<_>>()?;
        Ok(Versions { tags })
    }
}

// ----------------------------------------------------------------------------

impl Versions {
    /// Creates a range iterator over the version set.
    pub fn range<R>(&self, range: R) -> impl Iterator<Item = &Version>
    where
        R: RangeBounds<Version>,
    {
        // Compute range start
        let start = match range.start_bound() {
            Bound::Included(start) => Bound::Included(Reverse(start.clone())),
            Bound::Excluded(start) => Bound::Excluded(Reverse(start.clone())),
            Bound::Unbounded => Bound::Unbounded,
        };

        // Compute range end
        let end = match range.end_bound() {
            Bound::Included(end) => Bound::Included(Reverse(end.clone())),
            Bound::Excluded(end) => Bound::Excluded(Reverse(end.clone())),
            Bound::Unbounded => Bound::Unbounded,
        };

        // Return range iterator
        self.tags.range((start, end)).map(|v| &v.0)
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl fmt::Debug for Versions {
    /// Formats the version set for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Versions")
            .field("tags", &self.tags)
            .finish()
    }
}
