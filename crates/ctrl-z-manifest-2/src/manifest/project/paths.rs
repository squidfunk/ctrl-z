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

//! Path iterator.

use glob::glob;
use std::path::{Path, PathBuf};

use crate::manifest::{Manifest, Result};

use super::Project;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Path iterator.
#[derive(Debug, Default)]
pub struct Paths {
    /// Stack of patterns.
    patterns: Vec<PathBuf>,
    /// Stack of packages.
    packages: Vec<PathBuf>,
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl Paths {
    /// Creates a path iterator.
    ///
    /// Note that the given patterns must be valid paths and resolvable from
    /// the current working directory. It's recommended to use absolute paths.
    pub fn new<P>(patterns: P) -> Self
    where
        P: IntoIterator<Item = PathBuf>,
        P::IntoIter: DoubleEndedIterator,
    {
        Self {
            patterns: patterns.into_iter().rev().collect(),
            packages: Vec::new(),
        }
    }
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl Iterator for Paths {
    type Item = Result<PathBuf>;

    /// Returns the next path.
    fn next(&mut self) -> Option<Self::Item> {
        if self.packages.is_empty() {
            // Take next pattern from the stack, and expand it as a glob - if
            // the pattern is invalid, we propagate the error
            let paths = match glob(self.patterns.pop()?.to_str()?) {
                Ok(paths) => paths,
                Err(err) => return Some(Err(err.into())),
            };

            // Collect packages and propagate errors - note that we need to
            // know when an error occurs, so we don't just silence them
            let iter = paths.into_iter().map(|res| res.map_err(Into::into));
            match iter.collect::<Result<Vec<_>>>() {
                Ok(paths) => self.packages.extend(paths.into_iter().rev()),
                Err(err) => return Some(Err(err)),
            }
        }

        // Return next path
        self.packages.pop().map(Ok)
    }
}
