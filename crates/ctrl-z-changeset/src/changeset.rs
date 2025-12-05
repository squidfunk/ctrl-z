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

//! Changeset.

pub mod change;
pub mod changelog;
mod error;
pub mod revision;
pub mod scope;
pub mod version;

use change::Change;
use changelog::Changelog;
pub use error::{Error, Result};
use revision::Revision;
use scope::Scope;
use version::Increment;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Changeset.
///
/// Changesets extract information from commits, and associate them with a given
/// set of scopes. For each [`Scope`], an [`Increment`] is derived from changes
/// contained in the commits. This does not include transitive dependencies,
/// which are handled outside of changesets. Changesets only describe.
#[derive(Debug)]
pub struct Changeset<'a> {
    /// Scope set.
    scope: Scope,
    /// List of revisions.
    revisions: Vec<Revision<'a>>,
    /// Version increments.
    increments: Vec<Option<Increment>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<'a> Changeset<'a> {
    /// Creates a changeset.
    #[must_use]
    pub fn new(scope: Scope) -> Self {
        let increments = vec![None; scope.len()];
        Self {
            scope,
            revisions: Vec::default(),
            increments,
        }
    }

    /// Creates a changelog from the changeset.
    #[must_use]
    pub fn to_changelog(&'a self) -> Changelog<'a> {
        let mut changelog = Changelog::new(&self.scope);
        changelog.extend(&self.revisions);
        changelog
    }

    // @todo temp
    pub fn increments(&self) -> &[Option<Increment>] {
        &self.increments
    }
}

// #[allow(clippy::must_use_candidate)]
// impl Changeset<'_> {
//     /// Returns the scope set.
//     #[inline]
//     pub fn scope(&self) -> &Scope {
//         &self.scope
//     }

//     /// Returns the list of revisions.
//     #[inline]
//     pub fn revisions(&self) -> &[Revision<'_>] {
//         &self.revisions
//     }
// }
