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

//! Creates a new version and updates all packages.

use clap::Args;
use cliclack::log::remark;
use cliclack::{intro, outro, select};
use console::style;

use ctrl_z_changeset::VersionExt;
use ctrl_z_project::Cargo;
use ctrl_z_versioning::Manager;

use crate::cli::{Command, Result};
use crate::Options;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Creates a new version and updates all packages.
#[derive(Args, Debug)]
pub struct Arguments {}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Command for Arguments {
    /// Executes the command.
    fn execute(&self, options: Options) -> Result {
        //
        intro("")?;

        let mut manager = Manager::<Cargo>::new(options.directory)?;
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

                // Just keep the incrmen as is..
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

        manager.update(versions)?;

        // No errors occurred
        Ok(())
    }
}
