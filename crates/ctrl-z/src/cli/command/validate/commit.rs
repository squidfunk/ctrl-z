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

//! Validate a commit message.

use clap::Args;
use cliclack::{confirm, input, outro};
use std::fs;
use std::path::PathBuf;

use ctrl_z_changeset::changelog::Category;
use ctrl_z_changeset::Change;
use ctrl_z_project::Manifest;

use crate::cli::{Command, Result};
use crate::Options;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Validate a commit message.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Path to commit message file.
    file: Option<PathBuf>,
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
        let message = if let Some(ref file) = self.file {
            fs::read_to_string(file)?
        } else {
            use std::io::{self, Read};
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        };

        for line in message.lines() {
            let change: Change = match line.parse() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Invalid commit message line: {e}");
                    continue;
                }
            };
            if <Option<Category>>::from(&change).is_none() {
                continue;
            }

            // if file, then interactively - @todo
            if self.file.is_some() {
                let confirm2 = confirm("Is this commit related to an issue?")
                    .initial_value(true)
                    .interact()?;

                if confirm2 {
                    let issue: u32 = input("What's the number of the issue?")
                        .placeholder("e.g. 123")
                        .interact()?;

                    confirm("Does the commit resolve the issue?")
                        .initial_value(false)
                        .interact()?;
                }
            }

            // @todo: how to add it to the message?
            outro("Added ")?;
        }

        Ok(())
    }
}
