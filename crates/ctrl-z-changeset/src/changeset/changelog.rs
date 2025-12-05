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

//! Changelog.

use std::collections::BTreeMap;
use std::fmt;

use super::change::Kind;
use super::revision::Revision;
use super::scope::Scope;

mod section;

use section::{Category, Section};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Changelog.
pub struct Changelog<'a> {
    /// Changes grouped by category.
    sections: BTreeMap<Category, Section<'a>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<'a> Changelog<'a> {
    /// Creates a changelog from a scope and set of revisions.
    ///
    /// Note that only relevant changes are included in the changelog, which
    /// includes features, fixes, performance improvements and refactorings. In
    /// case the changeset does not include such changes, the changelog will be
    /// empty, which is expected, since no release is necessary.
    pub fn new<T>(iter: T, scope: &'a Scope) -> Self
    where
        T: IntoIterator<Item = &'a Revision<'a>>,
    {
        let mut sections = BTreeMap::<Category, Section>::new();
        for revision in iter {
            let change = revision.change();

            // Determine section title - not all types of changes are featured
            // in the changelog, so we skip those that are not relevant
            let category = if change.is_breaking() {
                Category::Breaking
            } else {
                match change.kind() {
                    Kind::Feature => Category::Feature,
                    Kind::Fix => Category::Fix,
                    Kind::Performance => Category::Performance,
                    Kind::Refactor => Category::Refactor,
                    _ => continue,
                }
            };

            // Retrieve or create section and add revision - note that we need
            // to pass the scopes for rendering, as only indices are stored
            sections
                .entry(category)
                .or_insert_with(|| category.into())
                .add(revision, scope);
        }

        // Return changelog
        Changelog { sections }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl fmt::Display for Changelog<'_> {
    /// Formats the changelog for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for section in self.sections.values() {
            section.fmt(f)?;
        }

        // No errors occurred
        Ok(())
    }
}
