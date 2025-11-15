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

//! Cargo manifest iterator.

use glob::glob;
use std::path::PathBuf;

use super::{Cargo, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Cargo manifest iterator.
pub struct Iter {
    /// Stack of members.
    members: Vec<PathBuf>,
    /// Stack of paths.
    paths: Vec<PathBuf>,
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl Iter {
    /// Creates a Cargo manifest iterator.
    pub fn new(cargo: &Cargo) -> Self {
        match cargo {
            Cargo::Package { .. } => Self::default(),
            Cargo::Workspace { workspace } => {
                let iter = workspace.members.iter().rev();
                Self {
                    members: iter
                        .map(|path| PathBuf::from(path).join("Cargo.toml"))
                        .collect(),
                    paths: Vec::new(),
                }
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl Iterator for Iter {
    type Item = Result<PathBuf>;

    /// Returns the next path.
    fn next(&mut self) -> Option<Self::Item> {
        if self.paths.is_empty() {
            // Take next member from the stack of members, and expand it as a
            // glob - if the pattern is invalid, propagate the error
            let paths = match glob(self.members.pop()?.to_str()?) {
                Ok(paths) => paths,
                Err(err) => return Some(Err(err.into())),
            };

            // Collect paths and propagate errors - note that we need to know
            // when an error occurs, so we don't just silence them
            let iter = paths.into_iter().map(|res| res.map_err(Into::into));
            match iter.collect::<Result<Vec<_>>>() {
                Ok(paths) => self.paths.extend(paths.into_iter().rev()),
                Err(err) => return Some(Err(err)),
            }
        }

        // Return next path
        self.paths.pop().map(Ok)
    }
}

// ----------------------------------------------------------------------------

impl Default for Iter {
    /// Creates a Cargo manifest iterator.
    fn default() -> Self {
        Self {
            members: Vec::new(),
            paths: Vec::new(),
        }
    }
}
