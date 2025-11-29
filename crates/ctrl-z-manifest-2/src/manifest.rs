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

//! Manifest.

use semver::Version;
use std::fs;
use std::path::{Path, PathBuf};

mod error;
pub mod format;
// mod iter;
// pub mod writer;

pub use error::{Error, Result};
use format::Format;
use iter::Iter;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Manifest.
#[derive(Debug)]
pub struct Manifest<F>
where
    F: Format,
{
    /// Manifest path.
    pub path: PathBuf,
    /// Manifest data.
    pub data: F,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<F> Manifest<F>
where
    F: Format,
{
    /// Attempts to read a manifest from the given path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Io`], if the manifest could not be read.
    pub fn read<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        Ok(Self {
            path: path.canonicalize()?,
            data: F::from_str(&content)?,
        })
    }

    /// Returns the manifest's name.
    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.data.name()
    }

    /// Returns the manifest's version.
    #[inline]
    pub fn version(&self) -> Option<&Version> {
        self.data.version()
    }
}

// paths should be iterated on manifest not on format... this is important
// because the path must be correctly resolved...

// Package, Workspace <- normalize,

struct Package {
    name: String,
    version: Version,
}

struct Workspace {
    members: Vec<String>, // ??? or Package | Workspace?
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<F> IntoIterator for Manifest<F>
where
    F: Format,
{
    type Item = Result<Manifest<F>>;
    type IntoIter = Iter<F>;

    /// Creates an iterator over the manifest.
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
