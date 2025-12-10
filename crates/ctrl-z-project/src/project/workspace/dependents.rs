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

//! Workspace dependents.

use std::ops::Index;
use zrx::graph::traversal::IntoIter;
use zrx::graph::Graph;

use crate::project::manifest::Manifest;
use crate::project::{Project, Result};

use super::Workspace;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Workspace dependents.
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

impl<T> Workspace<T>
where
    T: Manifest,
{
    /// Returns the dependents of a workspace.
    ///
    /// This method creates a graph that links projects in a workspace with
    /// their inner-workspace dependencies, allowing to perform a topological
    /// traversal. Handling packages in the right order it absolutely essential
    /// for correct versioning and release management.
    ///
    /// # Errors
    ///
    /// This method returns [`Error::Graph`][], if the graph could not be
    /// constructed, which should practically never happen.
    ///
    /// [`Error::Graph`]: crate::project::Error::Graph
    pub fn dependents(&self) -> Result<Dependents<'_, T>> {
        let mut builder = Graph::builder();

        // Collect all packages in the workspace, which are all projects that
        // have a dedicated name and version, and add them as nodes
        for project in self.projects.values() {
            if project.manifest.name().is_some() {
                builder.add_node(project);
            }
        }

        // Analyze dependencies between packages by iterating over all projects,
        // and adding edges for each dependency found in the workspace
        let mut edges = Vec::new();
        for (n, project) in builder.nodes().iter().enumerate() {
            for name in project.manifest.dependencies() {
                let Some(dependency) = self.get(name) else {
                    continue;
                };

                // Here, we're sure that this is a workspace project, so we can
                // look for the node index of the dependency, and add an edge
                // downstream from the project to its dependency. This ensures
                // that the graph can be traversed topologically.
                let mut iter = builder.nodes().iter();
                if let Some(m) = iter.position(|&next| next == dependency) {
                    edges.push((n, m));
                }
            }
        }

        // Create links between projects and their dependencies by adding all
        // collected edges to the graph
        for (n, m) in edges {
            builder.add_edge(m, n, ())?;
        }

        // Create and return dependents
        Ok(Dependents { graph: builder.build() })
    }
}

// ----------------------------------------------------------------------------

impl<T> Dependents<'_, T>
where
    T: Manifest,
{
    /// Creates an iterator over the dependents in the workspace.
    pub fn iter(&self) -> IntoIter {
        self.into_iter()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<'a, T> Index<usize> for Dependents<'a, T>
where
    T: Manifest,
{
    type Output = &'a Project<T>;

    /// Returns the project at the given index.
    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.graph[index]
    }
}

// ----------------------------------------------------------------------------

impl<'a, T> IntoIterator for &'a Dependents<'a, T>
where
    T: Manifest,
{
    type Item = usize;
    type IntoIter = IntoIter;

    /// Creates an iterator over the dependents in the workspace.
    fn into_iter(self) -> Self::IntoIter {
        self.graph.traverse(self.graph.sources()).into_iter()
    }
}
