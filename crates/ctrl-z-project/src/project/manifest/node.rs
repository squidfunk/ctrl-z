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

//! Node manifest.

use semver::{Version, VersionReq};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::project::manifest::Manifest;
use crate::project::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Node manifest.
///
/// Note that we only read parts of the manifest relevant to our use case, as
/// we're solely interested in identifying package name, version, and workspace
/// members, in order to bumping versions. Other fields can be safely ignored,
/// so we don't model them here.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: Version,
    /// Package workspace members.
    #[serde(default)]
    pub workspaces: Vec<String>,
    /// Package dependencies.
    #[serde(default)]
    pub dependencies: BTreeMap<String, VersionReq>,
    /// Package development dependencies.
    #[serde(default)]
    pub dev_dependencies: BTreeMap<String, VersionReq>,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Manifest for Node {
    /// Returns the name.
    #[inline]
    fn name(&self) -> Option<&str> {
        Some(&self.name)
    }

    /// Returns the version.
    #[inline]
    fn version(&self) -> Option<&Version> {
        Some(&self.version)
    }

    /// Returns the members.
    #[inline]
    fn members(&self) -> &[String] {
        &self.workspaces
    }
}

// ----------------------------------------------------------------------------

impl FromStr for Node {
    type Err = Error;

    /// Attempts to create a manifest from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self> {
        serde_json::from_str(value).map_err(Into::into)
    }
}
