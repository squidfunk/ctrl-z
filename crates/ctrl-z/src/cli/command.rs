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

//! Commands.

use clap::Subcommand;

use crate::cli::Result;
use crate::Options;

mod hook;
mod version;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

/// Command.
pub trait Command {
    /// Executes the command.
    fn execute(&self, options: Options) -> Result;
}

// ----------------------------------------------------------------------------
// Enums
// ----------------------------------------------------------------------------

/// Commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Versioning and release automation.
    Version {
        #[command(subcommand)]
        command: version::Commands,
    },
    /// Git hooks installation and usage.
    Hook {
        #[command(subcommand)]
        command: hook::Commands,
    },
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Command for Commands {
    /// Executes the command.
    fn execute(&self, options: Options) -> Result {
        match self {
            Commands::Version { command } => command.execute(options),
            Commands::Hook { command } => command.execute(options),
        }
    }
}
