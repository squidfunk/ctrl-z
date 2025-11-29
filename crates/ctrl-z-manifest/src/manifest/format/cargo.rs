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

//! Cargo manifest.

use semver::Version;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

use super::paths::Paths;
use super::{Error, Format, Result};

mod dependency;
mod package;
mod workspace;

pub use dependency::Dependency;
pub use package::Package;
pub use workspace::Workspace;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Cargo manifest.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Cargo {
    /// Cargo package.
    Package {
        /// Package information.
        package: Package,
        /// Package dependencies.
        #[serde(default)]
        dependencies: BTreeMap<String, Dependency>,
    },
    /// Cargo workspace.
    Workspace {
        /// Workspace information.
        workspace: Workspace,
    },
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Format for Cargo {
    /// Returns the manifest's name.
    #[inline]
    fn name(&self) -> Option<&str> {
        if let Cargo::Package { package, .. } = self {
            Some(&package.name)
        } else {
            None
        }
    }

    /// Returns the manifest's version.
    #[inline]
    fn version(&self) -> Option<&Version> {
        if let Cargo::Package { package, .. } = self {
            Some(&package.version)
        } else {
            None
        }
    }

    /// Creates an iterator over the manifest's paths.
    #[inline]
    fn paths(&self) -> Paths {
        if let Cargo::Workspace { workspace } = self {
            let iter = workspace.members.iter().rev();
            Paths::new(iter.map(|path| Path::new(path).join("Cargo.toml")))
        } else {
            Paths::default()
        }
    }
}

// ----------------------------------------------------------------------------

impl FromStr for Cargo {
    type Err = Error;

    /// Attempts to create a manifest from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self> {
        toml::from_str(value).map_err(Into::into)
    }
}
