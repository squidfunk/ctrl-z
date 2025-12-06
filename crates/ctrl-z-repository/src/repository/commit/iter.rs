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

//! Iterator over commits in a repository.

use git2::Oid;

use crate::repository::commit::Commit;
use crate::repository::{Repository, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over commits in a repository.
pub struct Commits<'a> {
    git_repository: &'a git2::Repository,
    /// Git diff object.
    git_revwalk: git2::Revwalk<'a>,
    /// Current index.
    index: usize,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    ///
    pub fn commits(&self) -> Result<Commits<'_>> {
        // Create a walk over all revisions starting from HEAD and walking
        // backwards topologically for as long as the iterator is consumed
        let mut revwalk = self.git_repository.revwalk()?;
        revwalk.push_head()?; // @todo start from another commit!
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

        Ok(Commits {
            git_repository: &self.git_repository,
            git_revwalk: revwalk,
            index: 0,
        })
    }

    // push commit, rather...
    pub fn commits_from(&self, oid: Oid) -> Result<Commits<'_>> {
        // Create a walk over all revisions starting from HEAD and walking
        // backwards topologically for as long as the iterator is consumed
        let mut revwalk = self.git_repository.revwalk()?;
        revwalk.push(oid)?; // @todo start from another commit!
        revwalk.set_sorting(git2::Sort::TOPOLOGICAL)?;

        Ok(Commits {
            git_repository: &self.git_repository,
            git_revwalk: revwalk,
            index: 0,
        })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a> Iterator for Commits<'a> {
    type Item = Result<Commit<'a>>;

    ///
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next delta from the diff
        let oid = self.git_revwalk.next()?;
        return match oid {
            Ok(oid) => Some(Commit::new(self.git_repository, oid)),
            Err(err) => Some(Err(err.into())),
        };
        None
    }
}
