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

//! Package.json manifest.

use semver::{Version, VersionReq};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::{Error, Result};

mod iter;

use iter::Iter;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Package.json manifest.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
    /// Package workspace members.
    pub workspaces: Option<Vec<String>>,
    /// Package dependencies.
    pub dependencies: Option<BTreeMap<String, VersionReq>>,
    /// Package development dependencies.
    pub dev_dependencies: Option<BTreeMap<String, VersionReq>>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl PackageJson {
    /// Attempts to load a Package.json manifest from the given path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Io`], if the manifest could not be read.
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let content = fs::read_to_string(path)?;
        Self::from_str(&content)
    }

    /// Creates an iterator over Cargo workspace members.
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter {
        Iter::new(self)
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for PackageJson {
    type Err = Error;

    /// Attempts to create a Package.json manifest from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self> {
        serde_json::from_str(value).map_err(Into::into)
    }
}

// ----------------------------------------------------------------------------

impl IntoIterator for &PackageJson {
    type Item = Result<PathBuf>;
    type IntoIter = Iter;

    /// Creates an iterator over Package.json workspace members.
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
