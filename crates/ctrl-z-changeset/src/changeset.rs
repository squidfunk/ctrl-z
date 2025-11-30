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
mod error;
pub mod revision;
pub mod scope;
pub mod version;

use change::Change;
pub use error::{Error, Result};
use revision::Revision;
use scope::Scope;

use crate::Increment;

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
    /// List of scopes.
    scope: Scope,
    /// Derived revisions.
    revisions: Vec<Revision<'a>>,
    /// Version increments.
    increments: Vec<Option<Increment>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Changeset<'_> {
    /// Creates a changeset.
    ///
    /// # Examples
    ///
    /// ```
    /// todo!()
    /// ```
    pub fn new(scope: Scope) -> Self {
        let increments = vec![None; scope.len()];
        Self {
            scope,
            revisions: Vec::new(),
            increments,
        }
    }

    // to_graph + to_changelog + to_plan?
}

// From the manifests, we extract all scopes.
// What do we need to update all manifests?
// - Scope -> Increment
// - Package name -> Version (Graph)
// - Apply increments to versions
// - Then update all manifests
