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

//! Iterator over changes in a commit.

use crate::git::Result;
use crate::git::change::Change;

use super::Commit;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over changes in a commit.
pub struct Changes<'a> {
    /// Git diff object.
    git_diff: git2::Diff<'a>,
    /// Current index.
    index: usize,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Commit<'_> {
    ///
    pub fn changes(&self) -> Result<Changes<'_>> {
        let tree = self.git_commit.tree()?;
        let parent_tree = if self.git_commit.parent_count() > 0 {
            Some(self.git_commit.parent(0)?.tree()?)
        } else {
            None
        };

        let mut diff_opts = git2::DiffOptions::new();
        let diff = self.git_repo.diff_tree_to_tree(
            parent_tree.as_ref(),
            Some(&tree),
            Some(&mut diff_opts),
        )?;

        Ok(Changes {
            git_diff: diff,
            index: 0, // Start at the first delta
        })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Iterator for Changes<'_> {
    type Item = Change;

    ///
    fn next(&mut self) -> Option<Self::Item> {
        // Get the next delta from the diff
        let delta = self.git_diff.get_delta(self.index)?;
        self.index += 1; // Increment the delta index

        // Determine the type of change
        match delta.status() {
            git2::Delta::Added => {
                // A new file was added
                let new_path = delta.new_file().path()?.to_path_buf();
                Some(Change::Create { path: new_path })
            }
            git2::Delta::Deleted => {
                // A file was deleted
                let old_path = delta.old_file().path()?.to_path_buf();
                Some(Change::Delete { path: old_path })
            }
            git2::Delta::Modified
            | git2::Delta::Copied
            | git2::Delta::Typechange => {
                // A file was modified
                let new_path = delta.new_file().path()?.to_path_buf();
                Some(Change::Modify { path: new_path })
            }
            git2::Delta::Renamed => {
                // A file was renamed
                let old_path = delta.old_file().path()?.to_path_buf();
                let new_path = delta.new_file().path()?.to_path_buf();
                Some(Change::Rename { from: old_path, path: new_path })
            }
            // Ignore other statuses (e.g., Unmodified, Ignored, Untracked)
            _ => self.next(),
        }
    }
}
