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

//! Scope builder.

use globset::{Glob, GlobSetBuilder};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::error::{Error, Result};
use super::Scope;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Scope builder.
#[derive(Debug)]
pub struct Builder {
    /// Registered paths.
    paths: BTreeMap<PathBuf, String>,
    /// Glob set builder.
    globs: GlobSetBuilder,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Builder {
    /// Creates a scope builder.
    ///
    /// Note that the canonical way to create a [`Scope`] is to invoke the
    /// [`Scope::builder`] method, which creates an instance of [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::scope::Builder;
    ///
    /// // Create scope builder
    /// let mut builder = Builder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            paths: BTreeMap::new(),
            globs: GlobSetBuilder::new(),
        }
    }

    /// Adds a path to the scope.
    ///
    /// # Errors
    ///
    /// This method returns an error if the [`Glob`][] cannot be built.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use ctrl_z_changeset::Scope;
    ///
    /// // Create scope builder and add path
    /// let mut builder = Scope::builder();
    /// builder.add("crates/ctrl-z", "ctrl-z")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add<P, N>(&mut self, path: P, name: N) -> Result<&mut Self>
    where
        P: AsRef<Path>,
        N: Into<String>,
    {
        let path = path.as_ref();
        if !path.is_relative() {
            return Err(Error::PathAbsolute);
        }

        // Ensure path does not already exist, as scopes can't overlap
        if self.paths.contains_key(path) {
            Err(Error::PathExists)

        // Create pattern matching all files under the given path
        } else {
            let glob = path.join("**");

            // Create glob and add to builder
            self.paths.insert(path.to_path_buf(), name.into());
            Glob::new(&glob.to_string_lossy())
                .map_err(Into::into)
                .map(|glob| {
                    self.globs.add(glob);
                    self
                })
        }
    }

    /// Builds the scope.
    ///
    /// # Errors
    ///
    /// This method returns an error if the [`GlobSet`][] cannot be built.
    ///
    /// [`GlobSet`]: globset::GlobSet
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use ctrl_z_changeset::Scope;
    ///
    /// // Create scope builder and add path
    /// let mut builder = Scope::builder();
    /// builder.add("crates/ctrl-z", "ctrl-z")?;
    ///
    /// // Create scope from builder
    /// let scope = builder.build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<Scope> {
        Ok(Scope {
            paths: self.paths.into_iter().collect(),
            globs: self.globs.build()?,
        })
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Default for Builder {
    /// Creates a scope builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::scope::Builder;
    ///
    /// // Create scope builder
    /// let mut builder = Builder::default();
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
