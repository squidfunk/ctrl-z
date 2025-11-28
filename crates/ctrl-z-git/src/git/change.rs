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

//! Change.

use std::path::PathBuf;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Change {
    /// Path was created.
    Create { path: PathBuf },
    /// Path was modified.
    Modify { path: PathBuf },
    /// Path was renamed.
    Rename { from: PathBuf, path: PathBuf },
    /// Path was deleted
    Delete { path: PathBuf },
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Change {
    /// Returns the path of the change.
    #[inline]
    #[must_use]
    pub fn path(&self) -> &PathBuf {
        match self {
            Change::Create { path, .. } => path,
            Change::Modify { path, .. } => path,
            Change::Rename { path, .. } => path,
            Change::Delete { path, .. } => path,
        }
    }
}
