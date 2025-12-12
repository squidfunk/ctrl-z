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
use std::fmt::Debug;

use ctrl_z_changeset::VersionExt;
use ctrl_z_project::Manifest;
use ctrl_z_versioning::Manager;

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
        let manager = Manager::<T>::new(options.directory)?;
        let changeset = manager.changeset(self.version.as_ref())?;

        // Create temporary vector for writing - as we need to be particularly
        // careful about line feeds, we first collect everything to write
        let mut temp = Vec::new();
        if self.summary {
            temp.push(changeset.summary()?);
        }

        // Only write to standard out if the changeset is not empty, in order
        // to mitigate writing of empty lines
        if !changeset.is_empty() {
            let changelog = changeset.to_changelog().to_string();
            temp.push(&changelog);

            // Write to standard out
            if !temp.is_empty() {
                println!("{}", temp.join("\n\n"));
            }
        }

        // No errors occurred
        Ok(())
    }
}
