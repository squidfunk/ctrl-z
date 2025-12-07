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

//! Repository.

use std::path::Path;

pub mod commit;
mod error;
pub mod id;
pub mod versions;

pub use error::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Repository.
pub struct Repository {
    /// Git repository.
    inner: git2::Repository,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Repository {
    /// Finds and open a repository starting from the given path.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Git`] if the operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// use ctrl_z_repository_2::Repository;
    /// use std::env;
    ///
    /// // Find and open repository from current directory
    /// let repo = Repository::open(env::current_dir()?)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        git2::Repository::discover(path)
            .map_err(Into::into)
            .map(|inner| Self { inner })
    }
}

#[allow(clippy::must_use_candidate)]
impl Repository {
    /// Returns the repository path.
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    pub fn path(&self) -> &Path {
        self.inner.path().parent().expect("invariant")
    }
}
