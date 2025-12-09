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
use std::fs;
use std::path::PathBuf;

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
    version: Option<Version>,
    /// Include version summary.
    #[arg(short, long)]
    summary: bool,
    /// Output to file.
    #[arg(short, long)]
    output: Option<PathBuf>,
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

        // Create changelog and prepend version summary if desired
        let mut changelog = changeset.to_changelog().to_string();
        if self.summary {
            let summary = changeset.summary().unwrap_or_default();
            changelog = format!("{summary}\n\n{changelog}");
        }

        // Write to standard out or file
        if let Some(output) = &self.output {
            // In order to be predictable and consistent, we always need to
            // write the changelog to a file, even though it may be empty
            fs::create_dir_all(output.parent().expect("invariant"))?;
            fs::write(output, changelog)?;
        } else if !changelog.is_empty() {
            println!("{changelog}");
        }

        // No errors occurred
        Ok(())
    }
}
