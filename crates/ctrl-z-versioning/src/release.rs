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

//! Version manager.

use semver::Version;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use ctrl_z_changeset::{Changeset, Increment, VersionExt};
use ctrl_z_project::workspace::updater::Updatable;
use ctrl_z_project::{Manifest, Workspace};
use ctrl_z_repository::Repository;

mod error;

pub use error::{Error, Result};

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Version manager.
#[derive(Debug)]
pub struct Manager<T>
where
    T: Manifest,
{
    /// Repository.
    repository: Repository,
    /// Workspace.
    workspace: Workspace<T>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

// @todo: do we need a separate error? unclear... maybe we move this to project+
// itslf, and also move the updater there...? but this would mean a dep to repo

impl<T> Manager<T>
where
    T: Manifest,
{
    /// Creates a version manager at the given path.
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let repository = Repository::open(path.as_ref())?;
        let path = T::resolve(repository.path())?;
        Ok(Self {
            repository,
            workspace: Workspace::read(path)?,
        })
    }

    // @todo with_options(dry run?)

    // we should move this iterator to the repository, then we dont need this
    // method...

    /// obtain changelog... - we should deduce version outside! otherwise, its ehead
    /// // @todo: maybe we should pass version in the options...
    pub fn changeset(
        &self, version: Option<&Version>,
    ) -> Result<Changeset<'_>> {
        let versions = self.repository.versions()?;

        // use version!
        let commits = if let Some(v) = version {
            if !versions.contains(v) {
                return Err(Error::Version(v.clone()));
            }

            // how can I find orphaned tags?

            let mut iter = versions.range(..=v).rev();
            let (v, start) = iter.next().expect("invariant"); // ok_or?
            let end = iter.next();
            // println!("start: {:?}, end: {:?}", v, end);
            if let Some((_, end)) = end {
                self.repository.commits(start..end)?
            } else {
                self.repository.commits(start..)?
            }
        } else {
            let mut iter = versions.range(..).rev();
            let end = iter.next();
            if let Some((_, end)) = end {
                self.repository.commits(..end)?
            } else {
                self.repository.commits(..)?
            }
        };

        // get summary from repository and add to changeset.
        // get canonical version – how? workspace config

        let mut changeset = Changeset::new(&self.workspace)?;
        changeset.extend(commits.flatten())?;
        Ok(changeset)
    }

    //
    pub fn changed(&self) {}

    // this should be called recommendation!?
    pub fn bump<F>(&self, mut f: F) -> Result<BTreeMap<String, Version>>
    where
        F: FnMut(
            &str,
            &Version,
            &[Option<Increment>],
        ) -> Result<Option<Increment>>,
    {
        // obtain the ids of all packages

        let changeset = self.changeset(None)?; // what if changeset is empty?
        let dependents = self.workspace.dependents()?;

        // increments are equal to the number of packaes, so we obtain the
        // index of the packages that changed, which are precisely the node
        // indices in the dependency graph
        let mut increments = changeset.increments().to_vec();
        let changed = increments
            .iter()
            .enumerate()
            .filter_map(|(index, increment)| increment.map(|_| index));

        // no dependencis?
        let incoming = dependents.graph.topology().incoming();
        for node in dependents.graph.traverse(changed) {
            let project = &dependents.graph[node];

            let name = project.name().expect("invariant");
            let current_version = project.version().expect("invariant");
            let bump = increments[node];

            // determine the max bump levels by deps. this dictates the number
            // of options that we currently have
            let mut dep_bump = BTreeSet::new();
            for &dep in &incoming[node] {
                dep_bump.insert(increments[dep]);
            }

            if dep_bump.is_empty() {
                // no deps! just use the exact bump!
                increments[node] = f(name, current_version, &[bump])?;
                // update the increment!
                continue;
            }

            // now, filter all things from dep_bump taht is smaller than the
            // current bump
            // println!("dep_bump before filter: {:?}", dep_bump);
            dep_bump.insert(bump); // insert earlier? or here?
            dep_bump.retain(|&b| b >= bump);

            // @todo handle max bumps!

            // if the original package is not bumped, we add all levels up to
            // the max bump / max dep bump.

            increments[node] = f(
                name,
                current_version,
                &dep_bump.into_iter().collect::<Vec<_>>(),
            )?;
        }

        // now, do the actual bumps? apply it to the worksapce...?
        // for project in self.workspace.iter_mut() {}

        let mut versions = BTreeMap::new();
        for (index, increment) in increments.iter().enumerate() {
            if let Some(increment) = increment {
                let project = &dependents.graph[index];
                let name = project.name().expect("invariant");
                let current_version = project.version().expect("invariant");

                let new_version = current_version.bump(*increment);
                versions.insert(name.to_string(), new_version);
            }
        }

        // @todo
        Ok(versions)
    }

    // we should use the versions thing as applyable to a workspace.
    // it should also consume the workspace
    pub fn update(
        &mut self, versions: BTreeMap<String, Version>, summary: String,
    ) -> Result<()>
    where
        T: Updatable,
    {
        // switch first!
        let version = "0.0.3";
        self.repository.branch(format!("release/v{version}"))?;

        for project in &mut self.workspace {
            project
                .update(
                    &versions
                        .iter()
                        .map(|(k, v)| (k.as_str(), v.clone()))
                        .collect(),
                )
                .unwrap();
        }

        // create new branch, commit to repo, then push everything.

        self.repository.add("*")?;

        // @todo: here we need to prompt for the release notes - add main tag
        self.repository
            .commit(format!("chore: release v{version}\n\n{summary}"))?;

        // commit message must contain changelog

        Ok(())
    }
}

#[allow(clippy::must_use_candidate)]
impl<T> Manager<T>
where
    T: Manifest,
{
    /// Returns a reference to the repository.
    #[inline]
    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    /// Returns a reference to the workspace.
    #[inline]
    pub fn workspace(&self) -> &Workspace<T> {
        &self.workspace
    }
}
