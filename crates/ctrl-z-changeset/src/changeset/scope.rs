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

//! Scope.

use globset::GlobSet;
use std::fmt;
use std::ops::Index;
use std::path::{Path, PathBuf};

mod builder;
mod error;

pub use builder::Builder;
pub use error::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Scope set.
///
/// Scopes are used to associate changes with non-overlapping paths in a git
/// repository, where a list of paths is matched through a [`GlobSet`]. When
/// two paths overlap, one path must be the prefix of another path. Thus, we
/// return the longer path as the matching scope.
#[derive(Clone)]
pub struct Scope {
    /// Registered path-name pairs.
    paths: Vec<(PathBuf, String)>,
    /// Glob set.
    globs: GlobSet,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Scope {
    /// Creates a scope set builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::Scope;
    ///
    /// // Create scope builder
    /// let mut builder = Scope::builder();
    #[inline]
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Returns the longest matching scope for the given path, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use ctrl_z_changeset::Scope;
    /// use std::path::Path;
    ///
    /// // Create scope builder and add path
    /// let mut builder = Scope::builder();
    /// builder.add("crates/ctrl-z", "ctrl-z")?;
    ///
    /// // Create scope from builder
    /// let scope = builder.build()?;
    ///
    /// // Create path and obtain longest matching scope
    /// let path = Path::new("crates/ctrl-z/Cargo.toml");
    /// assert_eq!(scope.matches(&path), Some(0));
    /// # Ok(())
    /// # }
    /// ```
    pub fn matches<P>(&self, path: P) -> Option<usize>
    where
        P: AsRef<Path>,
    {
        self.globs.matches(path).into_iter().max_by_key(|&index| {
            let (path, _) = &self.paths[index];
            path.components().count()
        })
    }
}

#[allow(clippy::must_use_candidate)]
impl Scope {
    /// Returns the number of scopes.
    #[inline]
    pub fn len(&self) -> usize {
        self.paths.len()
    }

    /// Returns whether there are any scopes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.paths.is_empty()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Index<usize> for Scope {
    type Output = (PathBuf, String);

    /// Returns the scope at the given index.
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.paths[index]
    }
}

// ----------------------------------------------------------------------------

impl fmt::Debug for Scope {
    /// Formats the scope set for debugging.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Scope")
            .field("paths", &self.paths)
            .finish_non_exhaustive()
    }
}
