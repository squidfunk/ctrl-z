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
use std::path::Path;
use std::str::FromStr;

use super::paths::Paths;
use super::{Error, Format, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Package.json manifest.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageJson {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
    /// Package private flag.
    pub private: bool,
    /// Package workspace members.
    pub workspaces: Option<Vec<String>>,
    /// Package dependencies.
    pub dependencies: Option<BTreeMap<String, VersionReq>>,
    /// Package development dependencies.
    pub dev_dependencies: Option<BTreeMap<String, VersionReq>>,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Format for PackageJson {
    /// Returns the manifest's name.
    #[inline]
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    /// Returns the manifest's version.
    #[inline]
    fn version(&self) -> Option<&Version> {
        Some(&self.version)
    }

    /// Creates an iterator over the manifest's paths.
    #[inline]
    fn paths(&self) -> Paths {
        if let Some(workspaces) = &self.workspaces {
            let iter = workspaces.iter().rev();
            Paths::new(iter.map(|path| Path::new(path).join("package.json")))
        } else {
            Paths::default()
        }
    }
}

// ----------------------------------------------------------------------------

impl FromStr for PackageJson {
    type Err = Error;

    /// Attempts to create a manifest from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self> {
        serde_json::from_str(value).map_err(Into::into)
    }
}
