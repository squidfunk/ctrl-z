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

//! Iterator over references in a repository.

use git2::string_array::StringArray;

use crate::repository::reference::Reference;
use crate::repository::{Repository, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over references in a repository.
pub struct References<'a> {
    git_repository: &'a git2::Repository,
    /// Git references iterator.
    git_references: git2::References<'a>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    /// Creates an iterator over all references (heads, tags, remotes, etc.).
    pub fn references(&self) -> Result<References<'_>> {
        let refs = self.git_repository.references()?;
        Ok(References {
            git_repository: &self.git_repository,
            git_references: refs,
        })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a> Iterator for References<'a> {
    type Item = Result<Reference<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.git_references.next()? {
            Ok(reference) => {
                Some(Ok(Reference::new(self.git_repository, reference)))
            }
            Err(err) => Some(Err(err.into())),
        }
    }
}
