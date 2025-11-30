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

use std::marker::PhantomData;

use super::error::Result;
use super::manifest::Manifest;
use super::Project;

mod paths;

use paths::Paths;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Members iterator.
///
/// @todo document that this is recursive
#[derive(Debug)]
pub struct Members<M> {
    /// Stack of path iterators.
    stack: Vec<Paths>,
    /// Type marker.
    marker: PhantomData<M>,
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl<M> Project<M>
where
    M: Manifest,
{
    /// Creates a members iterator.
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn members(&self) -> Members<M> {
        let root = self.path.parent().expect("invariant");
        let iter = self.data.members().iter().map(|path| root.join(path));
        Members {
            stack: vec![iter.rev().collect()],
            marker: PhantomData,
        }
    }
}

// ----------------------------------------------------------------------------
// Implementation
// ----------------------------------------------------------------------------

impl<M> Iterator for Members<M>
where
    M: Manifest,
{
    type Item = Result<Project<M>>;

    /// Returns the next manifest.
    fn next(&mut self) -> Option<Self::Item> {
        // check if we still have paths in the topmost iterator, and print.
        // otherw
        while let Some(stack) = self.stack.last_mut() {
            if let Some(res) = stack.next() {
                if let Ok(path) = res {
                    let path = path.join("Cargo.toml");

                    // println!("Members: stack={:#?}, x={:#?}", stack, path);

                    // Load and create next members iterator...
                    let project = match Project::<M>::read(path) {
                        Ok(project) => project,
                        Err(err) => return Some(Err(err)),
                    };

                    //
                    let members = project.members();
                    self.stack.extend(members.stack);

                    // Return project
                    return Some(Ok(project));
                }
            } else {
                self.stack.pop();
            }
        }

        None
    }
}
