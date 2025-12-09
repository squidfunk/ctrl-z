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

use ctrl_z_project::{Manifest, Workspace};

pub mod change;
pub mod changelog;
mod error;
pub mod revision;
pub mod scopes;
pub mod summary;
pub mod version;

use change::Change;
pub use error::{Error, Result};
use revision::Revision;
use scopes::Scopes;
use summary::Summary;
use version::Increment;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Changeset.
///
/// Changesets extract information from commits, and associate them with a given
/// set of scopes. For all [`Scopes`], an [`Increment`] is derived from changes
/// contained in the commits. This does not include transitive dependencies,
/// which are handled outside of changesets. Changesets only describe.
#[derive(Debug)]
pub struct Changeset<'a> {
    /// Scope set.
    scopes: Scopes,
    /// List of revisions.
    revisions: Vec<Revision<'a>>,
    /// Version increments.
    increments: Vec<Option<Increment>>,
    /// Summary of changes.
    summary: Option<Summary>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Changeset<'_> {
    /// Creates a changeset.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Scopes`][] if the scope set can't be built
    /// from the workspace, which should practically never happen.
    ///
    /// [`Error::Scopes`]: crate::changeset::Error::Scopes
    pub fn new<T>(workspace: &Workspace<T>) -> Result<Self>
    where
        T: Manifest,
    {
        let mut builder = Scopes::builder();
        for (path, name) in workspace.packages() {
            builder.add(path, name)?;
        }

        // Create scope set and version increments
        let scopes = builder.build()?;
        Ok(Self {
            increments: vec![None; scopes.len()],
            scopes,
            revisions: Vec::new(),
            summary: None,
        })
    }

    /// Updates the changeset with the given summary.
    #[must_use]
    pub fn with_summary(self, summary: Summary) -> Self {
        Self { summary: Some(summary), ..self }
    }
}

#[allow(clippy::must_use_candidate)]
impl Changeset<'_> {
    /// Returns a reference to the scope set.
    #[inline]
    pub fn scopes(&self) -> &Scopes {
        &self.scopes
    }

    /// Returns a reference to the list of revisions.
    #[inline]
    pub fn revisions(&self) -> &[Revision<'_>] {
        &self.revisions
    }

    /// Returns a reference to the version increments.
    #[inline]
    pub fn increments(&self) -> &[Option<Increment>] {
        &self.increments
    }

    /// Returns a reference to the summary.
    #[inline]
    pub fn summary(&self) -> Option<&Summary> {
        self.summary.as_ref()
    }

    /// Returns the number of revisions.
    #[inline]
    pub fn len(&self) -> usize {
        self.revisions.len()
    }

    /// Returns whether there are any revisions.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.revisions.is_empty()
    }
}
