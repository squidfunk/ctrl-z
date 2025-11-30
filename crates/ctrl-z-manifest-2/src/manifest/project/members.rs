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

//! Members iterator.

use std::path::{Path, PathBuf};

use crate::manifest::{Manifest, Result};

use super::Project;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Members iterator.
pub struct Members<M>
where
    M: Manifest,
{
    /// Stack of manifests.
    stack: Vec<(PathBuf, M)>,
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl<M> Project<M>
where
    M: Manifest,
{
    /// Creates a members iterator.
    #[inline]
    pub(crate) fn members(&self, path: PathBuf, root: M) -> Members<M> {
        Members { stack: vec![(path, root)] }
    }
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

// impl<M> Iterator for Members<M>
// where
//     M: Manifest,
// {
//     type Item = Result<(PathBuf, M)>;

//     /// Returns the next manifest.
//     fn next(&mut self) -> Option<Self::Item> {
//         let (path, manifest) = self.stack.pop()?;
//         // let data = &manifest.data;

//         // use path of manfiest as a base path!

//         // here, we can now use the members

//         // Collect paths and read manifests
//         let iter = manifest.paths(path).map(|res| res.and_then(Manifest::read));
//         let manifests = match iter.collect::<Result<Vec<_>>>() {
//             Ok(manifests) => manifests,
//             Err(err) => return Some(Err(err)),
//         };

//         // Add manifests to stack in pre-order
//         self.stack.extend(manifests.into_iter().rev());

//         // Return next manifest
//         Some(Ok(manifest))
//     }
// }
