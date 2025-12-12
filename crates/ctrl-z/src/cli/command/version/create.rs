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

//! Create a new version and updates all packages.

use clap::Args;
use cliclack::log::remark;
use cliclack::{intro, outro, select};
use console::style;
use std::io::Write;
use tempfile::NamedTempFile;

use ctrl_z_changeset::VersionExt;
use ctrl_z_project::Manifest;
use ctrl_z_versioning::Manager;

use crate::cli::{Command, Result};
use crate::Options;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Create a new version and updates all packages.
#[derive(Args, Debug)]
pub struct Arguments {
    /// Use visual editor for release notes.
    #[arg(short, long)]
    visual: bool,
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
        let mut manager = Manager::<T>::new(options.directory)?;
        // @todo: ensure everything is clean!! no uncommitted changes.

        let changeset = manager.changeset(None);
        println!("changeset: {:?}", changeset);

        //
        intro("")?;

        let versions = manager.bump(|name, version, bumps| {
            if bumps.len() == 1 {
                // @todo is the expext right here?
                let increment = bumps[0].expect("invariant");

                let x = format!(
                    "{}\n{}",
                    name,
                    style(version.bump(increment)).dim()
                ); // denote what bumped
                remark(x)?;

                // Just keep the incrmen as is.. - todo...
                return Ok(Some(increment));
            }

            //
            let mut builder =
                bumps.iter().fold(select(name), |builder, &bump| {
                    if let Some(next) = bump {
                        builder.item(Some(next), version.bump(next), next)
                    } else {
                        builder.item(None, version, "current")
                    }
                });

            //
            Ok(builder.interact()?)
        })?;

        outro("")?;

        println!("versions: {:?}", versions);

        let summary = prompt_commit_message(self.visual)?;

        // manager.update(versions, summary)?;

        // No errors occurred
        Ok(())
    }
}

fn prepend_lines_with_quote(text: &str) -> String {
    text.lines()
        .map(|line| format!("> {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn prompt_commit_message(visual: bool) -> Result<String> {
    // Create a temporary file with template - read template and include string
    // move this into our obligatory configuration file
    let mut temp = NamedTempFile::new()?;
    writeln!(temp, "## Summary\n\n...\n\n### Highlights\n\n- ...")?;

    // Get editor from environment or use default
    let editor = if visual {
        std::env::var("VISUAL").unwrap_or_else(|_| "vim".to_string())
    } else {
        std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string())
    };

    // Open editor
    let mut cmd = std::process::Command::new(&editor);
    cmd.arg(temp.path());
    if editor == "code" {
        cmd.arg("--wait");
    }
    let status = cmd.status()?;
    println!("status {:?}", status);
    // check if the file is the same actually
    if !status.success() {
        std::process::exit(1);
        // return Err("Editor exited with non-zero status".into());
    }

    // Read the message back
    let message = std::fs::read_to_string(temp.path())?;

    // // Filter out comment lines and trim
    // let message: String = content
    //     .lines()
    //     .filter(|line| !line.trim_start().starts_with('#'))
    //     .collect::<Vec<_>>()
    //     .join("\n")
    //     .trim()
    //     .to_string();

    if message.is_empty() {
        std::process::exit(1);
        // return Err("Empty commit message".into());
    }

    Ok(message)
}
