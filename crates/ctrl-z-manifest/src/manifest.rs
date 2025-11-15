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

use std::path::{Path, PathBuf};

pub mod cargo;
mod error;
mod iter;
pub mod npm;
mod paths;

use cargo::Cargo;
pub use error::{Error, Result};
use iter::Iter;
use npm::PackageJson;
use paths::Paths;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Manifest.
#[derive(Debug)]
pub enum Manifest {
    /// Cargo manifest.
    Cargo {
        /// Manifest path.
        path: PathBuf,
        /// Manifest data.
        data: Cargo,
    },

    /// Package.json manifest.
    PackageJson {
        /// Manifest path.
        path: PathBuf,
        /// Manifest data.
        data: PackageJson,
    },
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Manifest {
    /// Load manifest from path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Io`], if the manifest could not be read.
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        match path.file_name().and_then(|value| value.to_str()) {
            Some("Cargo.toml") => Ok(Manifest::Cargo {
                path: path.to_path_buf(),
                data: Cargo::new(path)?,
            }),
            Some("package.json") => Ok(Manifest::PackageJson {
                path: path.to_path_buf(),
                data: PackageJson::new(path)?,
            }),
            _ => Err(Error::Invalid),
        }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl IntoIterator for Manifest {
    type Item = Result<Manifest>;
    type IntoIter = Iter;

    /// Creates an iterator over the manifest.
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}
