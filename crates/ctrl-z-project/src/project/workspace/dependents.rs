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

//! Iterator over dependents in a workspace.

use zrx::graph::Graph;

use crate::project::manifest::Manifest;
use crate::project::{Project, Result};

use super::Workspace;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Iterator over dependents in a workspace.
#[derive(Debug)]
pub struct Dependents<'a, T>
where
    T: Manifest,
{
    /// Workspace graph.
    pub graph: Graph<&'a Project<T>>, // @todo pub temp
}

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

// pub struct

// we might just number the packages,

impl<T> Workspace<T>
where
    T: Manifest,
{
    /// @todo cleanup
    pub fn dependents(&self) -> Result<Dependents<'_, T>> {
        let mut builder = Graph::builder();
        for project in self.projects.values() {
            // If the project's manifest includes a name, we must treat it as a
            // dedicated package. This does not hold for workspaces in Rust.
            if project.manifest.name().is_some() {
                builder.add_node(project);
            }
        }

        let mut edges = Vec::new();
        let projects = builder.nodes();
        for (n, project) in projects.iter().enumerate() {
            for name in project.manifest.dependencies() {
                // find the dependency in the workspace
                if let Some(dependency) = self.get(name) {
                    // get the actual dependency
                    if let Some(m) = projects.iter().position(|candidate| {
                        candidate.manifest.name() == dependency.manifest.name()
                    }) {
                        edges.push((n, m));
                    }
                }
            }
        }

        for (n, m) in edges {
            builder.add_edge(m, n, ())?;
        }

        // this is only necessary for propagation of versions...
        // let builder = Graph::builder();
        let graph = builder.build();
        Ok(Dependents { graph })
    }

    // now, a function that emits a project and a version bump, and
    // expects to return another version bump, then returns all bumps?
}
