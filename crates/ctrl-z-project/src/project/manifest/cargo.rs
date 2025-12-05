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

use semver::{Version, VersionReq};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::project::manifest::{Dependencies, Manifest};
use crate::project::{Error, Result};

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
///
/// Note that we only read parts of the manifest relevant to our use case, as
/// we're solely interested in identifying package name, version, and workspace
/// members, in order to bumping versions. Other fields can be safely ignored,
/// so we don't model them here.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Cargo {
    /// Cargo workspace.
    Workspace {
        /// Workspace data.
        workspace: Workspace,
    },
    /// Cargo package.
    Package {
        /// Package data.
        package: Package,
        /// Package dependencies.
        #[serde(default)]
        dependencies: BTreeMap<String, Dependency>,
    },
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Manifest for Cargo {
    /// Returns the name.
    #[inline]
    fn name(&self) -> Option<&str> {
        if let Cargo::Package { package, .. } = self {
            Some(&package.name)
        } else {
            None
        }
    }

    /// Returns the version.
    #[inline]
    fn version(&self) -> Option<&Version> {
        if let Cargo::Package { package, .. } = self {
            Some(&package.version)
        } else {
            None
        }
    }

    /// Returns the members.
    #[inline]
    fn members(&self) -> &[String] {
        if let Cargo::Workspace { workspace } = self {
            &workspace.members
        } else {
            &[]
        }
    }
}

impl Dependencies for Cargo {
    fn dependencies(
        &self,
    ) -> impl Iterator<Item = (&String, Option<&VersionReq>)> {
        let dependencies = match self {
            Cargo::Package { dependencies, .. } => dependencies,
            Cargo::Workspace { workspace } => &workspace.dependencies,
        };

        dependencies.iter().map(|(name, dependency)| {
            let version = match dependency {
                Dependency::Version(version) => Some(version),
                Dependency::Info(info) => info.version.as_ref(),
            };
            (name, version)
        })
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
