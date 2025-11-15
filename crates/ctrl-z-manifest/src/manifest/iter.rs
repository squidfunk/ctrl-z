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

//! Manifest iterator.

use super::error::Result;
use super::Manifest;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Manifest iterator.
pub struct Iter {
    /// Stack of manifests.
    stack: Vec<Manifest>,
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl Iter {
    /// Creates a manifest iterator.
    pub fn new(manifest: Manifest) -> Self {
        Self { stack: vec![manifest] }
    }
}

// -> I: IntoIterator<Item = Result<PathBuf>>

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl Iterator for Iter {
    type Item = Result<Manifest>;

    /// Returns the next manifest.
    fn next(&mut self) -> Option<Self::Item> {
        let manifest = self.stack.pop()?;
        match &manifest {
            Manifest::Cargo { data, .. } => {
                let iter =
                    data.into_iter().map(|res| res.and_then(Manifest::new));

                // Collect and return manifests
                let manifests = match iter.collect::<Result<Vec<_>>>() {
                    Ok(manifests) => manifests,
                    Err(err) => return Some(Err(err)),
                };

                // Add manifests to stack for pre-order traversal
                self.stack.extend(manifests.into_iter().rev());
            }

            Manifest::PackageJson { data, .. } => {
                let iter =
                    data.into_iter().map(|res| res.and_then(Manifest::new));

                // Collect and return manifests
                let manifests = match iter.collect::<Result<Vec<_>>>() {
                    Ok(manifests) => manifests,
                    Err(err) => return Some(Err(err)),
                };

                // Add manifests to stack for pre-order traversal
                self.stack.extend(manifests.into_iter().rev());
            }
        }

        // Return next manifest
        Some(Ok(manifest))
    }
}
