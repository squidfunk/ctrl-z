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
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Bound, RangeBounds};
use std::str::FromStr;

use super::commit::Commit;
use super::error::Result;
use super::Repository;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Version set.
///
/// This data type represents a set of versions in a given repository. Versions
/// are ordered from new to old, so iteration and range queries are simple. Each
/// is mapped to the commit it tags, so those commits can be obtained to query
/// for changes between two versions.
pub struct Versions<'a> {
    /// Tagged versions.
    tags: BTreeMap<Reverse<Version>, Commit<'a>>,
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
    pub fn versions(&self) -> Result<Versions<'_>> {
        let tags = self.inner.tag_names(Some("v[0-9]*.[0-9]*.[0-9]**"))?;
        let iter = tags.iter().flatten().map(|name| {
            let version = Version::from_str(name.trim_start_matches('v'))?;
            Ok((Reverse(version), self.find(name)?))
        });

        // Collect and return version set
        let tags = iter.collect::<Result<_>>()?;
        Ok(Versions { tags })
    }
}

// ----------------------------------------------------------------------------

impl Versions<'_> {
    /// Creates a range iterator over the version set.
    pub fn range<R>(
        &self, range: R,
    ) -> impl Iterator<Item = (&Version, &Commit<'_>)>
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
        self.tags
            .range((start, end))
            .map(|(version, commit)| (&version.0, commit))
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl fmt::Debug for Versions<'_> {
    /// Formats the version set for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Versions")
            .field("tags", &self.tags)
            .finish()
    }
}
