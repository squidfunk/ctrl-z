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

use ctrl_z_changeset::change::Kind;
use ctrl_z_changeset::Changeset;

mod section;

use section::{Section, Title};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Changelog.
pub struct Changelog<'a> {
    /// Sections indexed by kind.
    sections: BTreeMap<Title, Section<'a>>,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a> From<&'a Changeset<'a>> for Changelog<'a> {
    /// Creates a changelog from a changeset.
    ///
    /// Note that only relevant changes are included in the changelog, which
    /// includes features, fixes, performance improvements and refactorings. In
    /// case the changeset does not include such changes, the changelog will be
    /// empty, which is expected, since no release is necessary.
    fn from(changeset: &'a Changeset<'a>) -> Self {
        let mut sections = BTreeMap::new();
        for revision in changeset.revisions() {
            let change = revision.change();

            // Determine section title - not all types of changes are featured
            // in the changelog, so we skip those that are not relevant
            let title = if change.is_breaking() {
                Title::Breaking
            } else {
                match change.kind() {
                    Kind::Feature => Title::Feature,
                    Kind::Fix => Title::Fix,
                    Kind::Performance => Title::Performance,
                    Kind::Refactor => Title::Refactor,
                    _ => continue,
                }
            };

            // Determine relevant section and add revision - note that we need
            // to pass the scopes for rendering, as only indices are stored
            sections
                .entry(title)
                .or_insert_with(|| Section::from(title))
                .add(revision, changeset.scope());
        }

        // Return changelog
        Changelog { sections }
    }
}

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
