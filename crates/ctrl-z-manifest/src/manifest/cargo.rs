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

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::manifest::paths::Paths;

use super::{Error, Result};

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
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Cargo {
    /// Cargo package.
    Package {
        /// Package information.
        package: Package,
        /// Package dependencies.
        dependencies: Option<BTreeMap<String, Dependency>>,
    },
    /// Cargo workspace.
    Workspace {
        /// Workspace information.
        workspace: Workspace,
    },
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Cargo {
    /// Attempts to load a Cargo manifest from the given path.
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
    pub fn iter(&self) -> Paths {
        match self {
            Cargo::Package { .. } => Paths::default(),
            Cargo::Workspace { workspace } => Paths::new(
                workspace
                    .members
                    .iter()
                    .rev()
                    .map(|path| PathBuf::from(path).join("Cargo.toml")),
            ),
        }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl FromStr for Cargo {
    type Err = Error;

    /// Attempts to create a Cargo manifest from a string.
    #[inline]
    fn from_str(value: &str) -> Result<Self> {
        toml::from_str(value).map_err(Into::into)
    }
}

// ----------------------------------------------------------------------------

// @todo: do we need this? rather call it members? use a trait?
impl IntoIterator for &Cargo {
    type Item = Result<PathBuf>;
    type IntoIter = Paths;

    /// Creates an iterator over Cargo workspace members.
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
