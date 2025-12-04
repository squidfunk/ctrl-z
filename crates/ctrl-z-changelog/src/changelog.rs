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

use ctrl_z_changeset::change;
use ctrl_z_changeset::Changeset;

mod section;

use section::{Kind, Section};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Changelog.
pub struct Changelog<'a> {
    /// Sections indexed by kind.
    sections: BTreeMap<Kind, Section<'a>>,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

// @todo also add version bumps...

impl<'a> From<&'a Changeset<'a>> for Changelog<'a> {
    ///
    fn from(changeset: &'a Changeset<'a>) -> Self {
        let mut sections = BTreeMap::new();

        //
        for revision in changeset.revisions() {
            let change = revision.change();

            // Determine section kind - not all types of changes are featured
            // in the changelog, so we skip those that are not relevant
            let kind = if change.is_breaking() {
                Kind::Breaking
            } else {
                match change.kind() {
                    change::Kind::Feature => Kind::Feature,
                    change::Kind::Fix => Kind::Fix,
                    change::Kind::Performance => Kind::Performance,
                    change::Kind::Refactor => Kind::Refactor,
                    _ => continue,
                }
            };

            // section from revisions?

            // Determine relevant section
            println!("{:?}", change.kind());
            let section =
                sections.entry(kind).or_insert_with(|| Section::new(kind));

            println!("{:?}", section);
        }

        Changelog { sections }
    }
}
