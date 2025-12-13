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

//! Generate the changelog of a version in Markdown format.

use clap::Args;
use semver::Version;

use ctrl_z_changeset::{Changeset, VersionExt};
use ctrl_z_project::{Manifest, Workspace};
use ctrl_z_repository::Repository;

use crate::cli::{Command, Result};
use crate::Options;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Generate the changelog of a version in Markdown format.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Version in x.y.z format
    #[arg(value_parser = Version::from_str_with_prefix)]
    version: Option<Version>,
    /// Include version summary.
    #[arg(short, long)]
    summary: bool,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<T> Command<T> for Arguments
where
    T: Manifest,
{
    /// Executes the command.
    fn execute(&self, options: Options<T>) -> Result {
        let repository = Repository::open(options.directory)?;
        let workspace = Workspace::<T>::resolve(repository.path())?;

        // Resolve versions and create changeset, and determine all commits
        // that are either part of the given version or yet unreleased
        let versions = repository.versions()?;
        let mut changeset = Changeset::new(&workspace)?;
        if let Some(version) = &self.version {
            changeset.extend(versions.commits(version)?.flatten())?;
        } else {
            changeset.extend(versions.unreleased()?.flatten())?;
        }

        // Create queue for writing to standard out - since we are particularly
        // careful about line feeds, we collect everything before writing
        let mut queue = Vec::new();
        if self.summary {
            queue.push(changeset.summary()?);
        }

        // Only write to standard out if the changelog is not empty, which can
        // happen despite changes being present - this happens when changes do
        // not touch published artifacts, as they solely improve on formatting,
        // documentation, or the build setup.
        let changelog = changeset.to_changelog();
        if !changelog.is_empty() {
            let value = changelog.to_string();
            queue.push(&value);

            // Got something to say
            if !queue.is_empty() {
                println!("{}", queue.join("\n\n"));
            }
        }

        // No errors occurred
        Ok(())
    }
}
