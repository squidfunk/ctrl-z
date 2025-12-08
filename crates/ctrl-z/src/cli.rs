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

//! Command line interface.

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::Parser;
use std::env;
use std::path::PathBuf;

mod command;

use command::{Command, Commands, Result};

// ----------------------------------------------------------------------------
// Constants
// ----------------------------------------------------------------------------

/// Command line styles.
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Global options.
struct Options {
    /// Configuration file.
    config: Option<PathBuf>,
    /// Working directory.
    directory: Option<PathBuf>,
}

// ----------------------------------------------------------------------------

/// Command line interface.
#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"))]
#[command(about = env!("CARGO_PKG_DESCRIPTION"), long_about = None)]
#[command(disable_help_subcommand = true)]
#[command(styles = STYLES)]
struct Cli {
    /// Configuration file.
    #[arg(short, long, global = true, default_value = ".ctrl-z.toml")]
    config: Option<PathBuf>,
    /// Working directory.
    #[arg(short, long, global = true)]
    directory: Option<PathBuf>,
    /// Commands.
    #[command(subcommand)]
    command: Commands,
}

// ----------------------------------------------------------------------------
// Program
// ----------------------------------------------------------------------------

/// Entry point.
fn main() -> Result {
    let cli = Cli::parse();
    cli.command.execute(Options {
        config: cli.config,
        directory: cli.directory,
    })
}
