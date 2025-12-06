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

//! Commit.

use git2::Oid;

use crate::repository::{Repository, Result};

mod iter;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Commit.
pub struct Commit<'a> {
    /// Repository.
    repository: &'a git2::Repository,
    /// Inner commit.
    inner: git2::Commit<'a>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    pub fn find(&self, oid: Oid) -> Result<Commit<'_>> {
        let inner = self.inner.find_commit(oid)?;
        Ok(Commit { repository: &self.inner, inner })
    }

    pub fn find_short<S>(&self, sha: S) -> Result<Commit<'_>>
    where
        S: AsRef<str>,
    {
        let inner = self.inner.find_commit_by_prefix(sha.as_ref())?;
        Ok(Commit { repository: &self.inner, inner })
    }
}

// ----------------------------------------------------------------------------

#[allow(clippy::must_use_candidate)]
impl Commit<'_> {
    /// Returns the commit identifier.
    #[inline]
    pub fn id(&self) -> git2::Oid {
        self.inner.id()
    }
}
