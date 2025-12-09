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

//! List the names of changed packages in topological order.

use clap::Args;
use semver::Version;
use std::collections::BTreeSet;
use std::path::PathBuf;

use ctrl_z_project::Manifest;
use ctrl_z_versioning::Manager;

use crate::cli::{Command, Result};
use crate::Options;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// List the names of changed packages in topological order.
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

impl<T> Command<T> for Arguments
where
    T: Manifest,
{
    /// Executes the command.
    fn execute(&self, options: Options<T>) -> Result {
        let manager = Manager::<T>::new(options.directory)?;
        let changeset = manager.changeset(self.version.as_ref())?;

        // collect all scopes
        let mut scopes = BTreeSet::<usize>::new(); // why?
        for revision in changeset.revisions() {
            scopes.extend(revision.scopes());
        }

        // create deps and determine correct order – @todo alarm when
        // no packages is part of a bump _ just don't do a release!
        let deps = manager.workspace().dependents().unwrap();
        let increments = changeset.increments();

        // traverse all nodes
        let mut traversal = deps.graph.traverse(deps.graph.sources());
        while let Some(node) = traversal.take() {
            if scopes.contains(&node) && increments[node].is_some() {
                let name = deps.graph[node].info().unwrap().0;
                println!("{name}");
            }
            let _ = traversal.complete(node);
        }

        // @todo add file output
        Ok(())
    }
}
