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

//! Section item.

use std::fmt::{self, Write};

use ctrl_z_changeset::{Revision, Scope};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Section item.
#[derive(Debug, PartialEq, Eq)]
pub struct Item<'a> {
    /// Commit identifier (short).
    id: &'a str,
    /// Affected scopes.
    scopes: Vec<&'a str>,
    /// Description.
    description: &'a str,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<'a> Item<'a> {
    // pub fn new(revision: &'a Revision<'a>, scope: &'a Scope) {
    //     let id = revision.commit().id().as_bytes();
    //     let id = &id[0..7];

    //     // there ... must be a description?
    //     revision.commit().summary();
    //     // x.
    //     // let scopes = scope.scopes().iter().map(|s| s.as_str()).collect();
    //     // let description = revision.description();

    //     Self { id, scopes, description }
    // }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl fmt::Display for Item<'_> {
    /// Formats the section kind for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.id)?;

        // Write scopes, if any
        if !self.scopes.is_empty() {
            f.write_char(' ')?;
            for (i, scope) in self.scopes.iter().enumerate() {
                f.write_str("__")?;
                f.write_str(scope)?;
                f.write_str("__")?;

                // Write comma if not last
                if i < self.scopes.len() - 1 {
                    f.write_str(", ")?;
                }
            }
        }

        // Write description
        f.write_str(" – ")?;
        f.write_str(self.description)
    }
}
