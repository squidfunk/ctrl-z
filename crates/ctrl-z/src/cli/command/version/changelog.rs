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

//! Generates a version's changelog.

use clap::Args;
use semver::Version;
use std::fs;
use std::path::PathBuf;

use ctrl_z_project::Cargo;
use ctrl_z_release::Release;

use crate::cli::{Command, Result};
use crate::Options;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Generates the changelog.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Version in x.y.z format
    version: Option<Version>,
    /// Output to file.
    #[arg(short, long)]
    output: Option<PathBuf>,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Command for Arguments {
    /// Executes the command.
    fn execute(&self, options: Options) -> Result {
        let release = Release::<Cargo>::new(options.directory)?;
        let changeset = release.changeset(self.version.as_ref())?;

        // Write changelog to standard out or file
        let changelog = changeset.to_changelog();
        if let Some(output) = &self.output {
            fs::create_dir_all(output.parent().expect("invariant"))?;
            fs::write(output, changelog.to_string())?;
        } else {
            print!("{changelog}");
        }

        // No errors occurred
        Ok(())
    }
}
