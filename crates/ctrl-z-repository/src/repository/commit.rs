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

use git2::{Oid, Repository};
use std::fmt;

use super::Result;

mod delta;
mod iter;

pub use delta::Delta;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Commit.
pub struct Commit<'a> {
    /// Repository.
    repository: &'a Repository,
    /// Inner commit.
    inner: git2::Commit<'a>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<'a> Commit<'a> {
    // @todo construct this with out ::new
    /// Loads the commit associated with the given object identifier.
    /// // @todo rather say repo.commit(oid?)
    pub fn new(repo: &'a Repository, oid: Oid) -> Result<Self> {
        let git_commit = repo.find_commit(oid)?; // is this a good abstraction?
        Ok(Self {
            repository: repo, // we need this for the deltas...
            inner: git_commit,
        })
    }

    #[inline]
    pub fn id(&self) -> Oid {
        self.inner.id()
    }

    #[inline]
    pub fn summary(&self) -> &str {
        self.inner.summary().expect("invariant")
    }

    #[inline]
    pub fn body(&self) -> Option<&str> {
        self.inner.body().filter(|body| !body.is_empty())
    }
}

impl PartialEq for Commit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.inner.id() == other.inner.id()
    }
}

impl Eq for Commit<'_> {}

impl fmt::Debug for Commit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Commit")
            .field("id", &self.inner.id())
            .field("summary", &self.inner.summary())
            .finish()
    }
}

// @todo also implement display! so we can print commits for debugging in the CLI
