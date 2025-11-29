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

//! Revision.

use ctrl_z_repository::Commit;
use std::collections::HashSet;
use std::str::FromStr;

use super::change::Change;
use super::error::Result;
use super::Changeset;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

// Revision.
#[derive(Debug, PartialEq, Eq)]
pub struct Revision<'a> {
    /// Original commit.
    commit: Commit<'a>,
    /// Computed change.
    change: Change,
    /// Affected scopes.
    scopes: HashSet<usize>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<'a> Changeset<'a> {
    /// Adds a commit to the changeset.
    ///
    /// # Errors
    ///
    /// This methods returns [`Error::Repository`][] if the commit deltas can't
    /// be retrieved. If the commit message couldn't be parsed, it will just be
    /// ignored, since there are several types of commits that will not make it
    /// into the changeset, e.g., merge commits.
    ///
    /// [`Error::Repository`]: crate::changeset::Error::Repository
    #[allow(clippy::missing_panics_doc)]
    pub fn add(&mut self, commit: Commit<'a>) -> Result {
        let summary = commit.summary().expect("invariant");
        if let Ok(change) = Change::from_str(summary) {
            // Retrieve affected scopes from commit
            let mut scopes = HashSet::new();
            for delta in commit.deltas()? {
                scopes.extend(self.scope.matches(delta.path()));
            }

            // Create revision and add to changeset
            self.revisions.push(Revision { commit, change, scopes });
        }

        // No errors occurred
        Ok(())
    }

    /// Extends the changeset with the given commits
    ///
    /// Note that we can't just implement the [`Extend`] trait, since addition
    /// of commits can fail due to parsing errors.
    ///
    /// # Errors
    ///
    /// This methods returns [`Error::Repository`][] if a commit's deltas can't
    /// be retrieved. If a commit's message couldn't be parsed, it will just be
    /// ignored, since there are several types of commits that will not make it
    /// into the changset, e.g., merge commits.
    ///
    /// [`Error::Repository`]: crate::changeset::Error::Repository
    pub fn extend<T>(&mut self, iter: T) -> Result
    where
        T: IntoIterator<Item = Commit<'a>>,
    {
        for commit in iter {
            self.add(commit)?;
        }

        // No errors occurred
        Ok(())
    }
}
