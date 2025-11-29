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
use std::path::Path;

mod changes;

use super::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Commit.
pub struct Commit<'a> {
    /// Repository.
    git_repo: &'a Repository,
    /// Inner commit.
    git_commit: git2::Commit<'a>,
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
        Ok(Self { git_repo: repo, git_commit })
    }

    #[inline]
    pub fn id(&self) -> Oid {
        self.git_commit.id()
    }

    #[inline]
    pub fn summary(&self) -> Option<&str> {
        self.git_commit.summary()
    }

    // pub fn author(&self) -> Option<&str> {
    //     // provide an author struct here as well!
    //     // self.git_commit.author().name()
    // }
}

impl PartialEq for Commit<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.git_commit.id() == other.git_commit.id()
    }
}

impl Eq for Commit<'_> {}
