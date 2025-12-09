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

//! Versioning.

use clap::Subcommand;

use crate::cli::{Command, Result};
use crate::Options;

mod changed;
mod changelog;
mod create;

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Versioning.
#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new version.
    Create(create::Arguments),
    /// Generates a version's changelog.
    Changelog(changelog::Arguments),
    /// Returns the names of changed packages.
    Changed(changed::Arguments),
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Command for Commands {
    /// Executes the command.
    fn execute(&self, options: Options) -> Result {
        match self {
            Commands::Changed(args) => args.execute(options),
            Commands::Changelog(args) => args.execute(options),
            Commands::Create(args) => args.execute(options),
        }
    }
}
