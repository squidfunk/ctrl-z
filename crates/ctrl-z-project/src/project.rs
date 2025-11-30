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

//! Project.

use std::fs;
use std::path::{Path, PathBuf};

mod error;
pub mod manifest;
mod members;

pub use error::{Error, Result};
use manifest::Manifest;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Project.
#[derive(Debug)]
pub struct Project<M>
where
    M: Manifest,
{
    /// Project path.
    pub path: PathBuf,
    /// Project manifest.
    pub data: M,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<M> Project<M>
where
    M: Manifest,
{
    /// Attempts to read a project from the given path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Io`], if the project could not be read.
    pub fn read<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        Ok(Self {
            path: path.canonicalize()?,
            data: M::from_str(&content)?,
        })
    }
}
