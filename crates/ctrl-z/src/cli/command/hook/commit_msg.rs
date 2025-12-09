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

//! Validates commit message format.

use clap::Args;
use cliclack::{confirm, input, intro, outro, select};
use ctrl_z_changeset::change::Kind;
use std::fs;
use std::path::PathBuf;

use ctrl_z_changeset::Change;

use crate::cli::{Command, Result};
use crate::Options;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Validates commit message format.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Path to commit message file.
    file: PathBuf,
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Command for Arguments {
    /// Executes the command.
    fn execute(&self, options: Options) -> Result {
        let message = fs::read_to_string(&self.file)?;

        let line = message.lines().next().expect("invariant");

        let change: Change = line.parse()?;

        // don't check issue on these kinds of changes
        match change.kind() {
            Kind::Feature => {}
            Kind::Fix => {}
            Kind::Performance => {}
            Kind::Refactor => {}
            _ => return Ok(()),
        }

        // confirm("Is this commit related to an issue?")

        // intro("Commit message validation")?;
        let confirm2 = confirm("Is this commit related to an issue?")
            .initial_value(true)
            .interact()?;

        if confirm2 {
            let issue: u32 = input("What's the number of the issue?") // fmt
                .placeholder("e.g. 123")
                .interact()?;

            confirm("Does the commit resolve the issue?")
                .initial_value(false)
                .interact()?;
        }

        outro("Added ")?;

        // ensure commit is signed...

        // let

        // load the file!
        Ok(())
    }
}
