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
use std::collections::btree_map::Range;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::RangeBounds;

use super::commit::Commits;
use super::error::Result;
use super::id::Id;
use super::Repository;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Version set.
///
/// This data type manages the existing versions in a given repository. Versions
/// are ordered chronologically, so iteration and range queries are simple. Each
/// version is mapped to the identifier of its corresponding commit, so commits
/// can be obtained to query for changes between two versions.
pub struct Versions<'a> {
    /// Repository.
    repository: &'a Repository,
    /// Versions and their corresponding commit identifiers.
    tags: BTreeMap<Version, Id>,
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
    /// build suffixes as well. Each tag is parsed as a [`Version`].
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`][] if the operation fails.
    ///
    /// [`Error::Git`]: crate::repository::Error::Git
    pub fn versions(&self) -> Result<Versions<'_>> {
        let tags = self.inner.tag_names(Some("v[0-9]*.[0-9]*.[0-9]**"))?;
        let iter = tags.iter().flatten().map(|name| {
            let version = name.trim_start_matches('v').parse()?;
            Ok((version, self.find(name)?.id()))
        });

        // Collect and return version set
        let tags = iter.collect::<Result<_>>()?;
        Ok(Versions { repository: self, tags })
    }
}

// ----------------------------------------------------------------------------

impl Versions<'_> {
    /// Returns the commit identifier for the given version.
    #[inline]
    #[must_use]
    pub fn get(&self, version: &Version) -> Option<&Id> {
        self.tags.get(version)
    }

    /// Returns whether the version set contains the given version.
    #[inline]
    #[must_use]
    pub fn contains(&self, version: &Version) -> bool {
        self.tags.contains_key(version)
    }

    /// Creates a range iterator over the version set.
    #[inline]
    pub fn range<R>(&self, range: R) -> Range<'_, Version, Id>
    where
        R: RangeBounds<Version>,
    {
        // we don't range iter over the version set BUT over the commits!
        self.tags.range(range)
    }

    // /// Returns an iterator over the commits at the head of the repository.
    // pub fn head(&self) -> Result<Commits<'_>> {
    //     // @todo maybe we should rename versions

    // }

    // Select? get?

    // iterator over commit in specific version
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
