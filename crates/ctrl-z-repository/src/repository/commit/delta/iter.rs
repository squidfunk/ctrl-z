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

//! Iterator over deltas in a commit.

use crate::repository::commit::{Commit, Delta};
use crate::repository::Result;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over deltas in a commit.
///
///
pub struct Deltas<'a> {
    /// Inner diff object.
    inner: git2::Diff<'a>,
    /// Current index.
    index: usize,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

// @todo rename this into Deltas? + Delta? not an event but a delta
impl Commit<'_> {
    ///
    pub fn deltas(&self) -> Result<Deltas<'_>> {
        let tree = self.inner.tree()?;
        let parent_tree = if self.inner.parent_count() > 0 {
            Some(self.inner.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_opts = git2::DiffOptions::new();
        let diff = self.repository.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut diff_opts),
        )?;

        Ok(Deltas {
            inner: diff,
            index: 0, // Start at the first delta
        })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Iterator for Deltas<'_> {
    type Item = Delta;

    ///
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next delta from the diff
        let delta = self.inner.get_delta(self.index)?;
        self.index += 1; // Increment the delta index

        //
        let from = delta.old_file().path()?;
        let path = delta.new_file().path()?;

        // Determine the type of change
        match delta.status() {
            git2::Delta::Added => {
                Some(Delta::Create { path: path.to_path_buf() })
            }
            // A file was deleted
            git2::Delta::Deleted => {
                Some(Delta::Delete { path: from.to_path_buf() })
            }
            // A file was modified
            git2::Delta::Modified
            | git2::Delta::Copied
            | git2::Delta::Typechange => {
                Some(Delta::Modify { path: path.to_path_buf() })
            }
            // A file was renamed
            git2::Delta::Renamed => Some(Delta::Rename {
                from: from.to_path_buf(),
                path: path.to_path_buf(),
            }),
            // Ignore other statuses (e.g., Unmodified, Ignored, Untracked)
            _ => self.next(),
        }
    }
}
