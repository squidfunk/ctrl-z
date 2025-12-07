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

//! Release.

use ctrl_z_changeset::Changeset;
use semver::Version;
use std::path::Path;

use ctrl_z_project::manifest::Resolver;
use ctrl_z_project::{Manifest, Workspace};
use ctrl_z_repository::Repository;

mod error;

pub use error::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Release manager.
#[derive(Debug)]
pub struct Release<T>
where
    T: Manifest,
{
    /// Repository.
    repository: Repository,
    /// Workspace.
    workspace: Workspace<T>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Release<T>
where
    T: Manifest + Resolver,
{
    /// Creates a release manager at the given path.
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let repository = Repository::open(path.as_ref())?;
        let path = T::resolve(repository.path())?;
        Ok(Self {
            repository,
            workspace: Workspace::read(path)?,
        })
    }

    /// obtain changelog... - we should deduce version outside! otherwise, its ehead
    /// // @todo: maybe we should pass version in the options...
    pub fn changeset(&self, version: Option<Version>) -> Result<Changeset<'_>> {
        let versions = self.repository.versions()?;

        // use version!
        let commits = if let Some(v) = version {
            if !versions.contains(&v) {
                return Err(Error::Version(v));
            }

            let mut iter = versions.range(v..);
            let (_, start) = iter.next().expect("invariant"); // ok_or?
            let end = iter.next();
            if let Some((_, end)) = end {
                self.repository.commits(start..end)?
            } else {
                self.repository.commits(start..)?
            }
        } else {
            let mut iter = versions.range(..);
            let end = iter.next();
            if let Some((_, end)) = end {
                self.repository.commits(..end)?
            } else {
                self.repository.commits(..)?
            }
        };

        let mut changeset = Changeset::new(&self.workspace)?;
        changeset.extend(commits.flatten())?;
        Ok(changeset)
    }
}

// @todo with_options(dry run?)
