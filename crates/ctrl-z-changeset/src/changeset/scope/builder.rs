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
use std::path::{Path, PathBuf};

use crate::changeset::Result;

use super::Scope;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Scope builder.
#[derive(Clone, Debug)]
pub struct Builder {
    /// Registered paths.
    paths: Vec<PathBuf>,
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
            paths: Vec::new(),
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
    /// builder.add("crates/ctrl-z-git")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn add<P>(&mut self, path: P) -> Result<&mut Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let glob = format!("{}/**", path.to_string_lossy());

        // Create glob and add to builder
        self.paths.push(path.to_path_buf());
        self.globs.add(Glob::new(&glob)?);

        // Return builder for chaining
        Ok(self)
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
    /// builder.add("crates/ctrl-z-git")?;
    ///
    /// // Create scope from builder
    /// let scope = builder.build()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<Scope> {
        Ok(Scope {
            paths: self.paths,
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
