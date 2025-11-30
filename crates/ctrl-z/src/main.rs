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

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::{Parser, Subcommand};
// @todo: remove the git indirection
use globset::Glob;
use inquire::Confirm;
use semver::{Version, VersionReq};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{cmp, env, fs, io};
use zrx::graph::Graph;

use ctrl_z_changeset::change::Kind;
use ctrl_z_changeset::{Change, Changeset, Scope, VersionExt};
use ctrl_z_manifest::{Cargo, Format, Manifest, PackageJson, Writer};
use ctrl_z_project::Manifest as _;
use ctrl_z_repository::Reference;
use ctrl_z_repository::{Commit, Repository};

// ----------------------------------------------------------------------------
// Constants
// ----------------------------------------------------------------------------

/// Configuration for command line styles.
const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .usage(AnsiColor::Green.on_default().effects(Effects::BOLD))
    .literal(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
    .placeholder(AnsiColor::Cyan.on_default());

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Command line interface.
#[derive(Parser)]
#[command(name = "ctrl-z")]
#[command(about = "tbd", long_about = None)]
#[command(styles=STYLES)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new tag
    Tag {
        /// Perform a dry run without making changes
        #[arg(long)]
        dry_run: bool,
    },
}

pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Tag { dry_run } => {
            if dry_run {
                println!("Dry run: no changes will be made");
            } else {
                println!("The following actions will be performed:");
                println!("- Create a new tag");

                let repo =
                    Repository::open(env::current_dir().unwrap()).unwrap();

                let graph = find_packages(repo.path());
                find_packages2(repo.path());

                return;

                // Build scope matcher
                let mut builder = Scope::builder();
                let root = repo.path();
                for meta in &graph {
                    let path =
                        meta.path.parent().unwrap().strip_prefix(root).unwrap();
                    println!("Adding scope for path: {:?}", path);
                    builder.add(path);
                }
                let scopes = builder.build().unwrap();

                // Determine LAST version that we released = last tag.
                let last_ref = if let Some(last) = repo
                    .references()
                    .unwrap()
                    .flatten()
                    .filter(Reference::is_tag)
                    .next()
                {
                    last
                } else {
                    return;
                };

                println!(
                    "Last reference: {:?}",
                    last_ref.commit().unwrap().unwrap().id()
                );

                let last_commit = last_ref.commit().unwrap().unwrap();

                // @todo changeset... - we can just create this from scopes!
                // Changeset::from(...)?
                let mut changeset = Changeset::new(scopes);
                let commits = repo
                    .commits()
                    .unwrap()
                    .flatten()
                    .take_while(|commit| commit != &last_commit);

                changeset.extend(commits).unwrap();

                println!("Changeset: {:#?}", changeset);

                // to_graph // to_changelog

                // @todo: changeset now has all increments!
                // we now need to compute the graph and propagate the increments
                // and apply the versions

                // // also do transitive bumps. can we do this by traversing a
                // // graph upward? downward? we should define it downward.

                // let root =
                //     Manifest::<Cargo>::read(repo.path().join("Cargo.toml"))
                //         .unwrap();

                // // println!("Top-level manifest: {:#?}", manifest);

                // // 3) Ensure nothing else is left dirty - move this to the top!
                // ensure_clean_workdir(repo.raw(), &[]).unwrap();

                // println!("Increments: {:#?}", increments);
                // // next, determine package versions and compute next ones
                // for i in 0..increments.len() {
                //     let manifest = &graph[i];
                //     let Some(increment) = increments[i] else {
                //         continue;
                //     };

                //     let Some(current_version) = manifest.data.version() else {
                //         continue;
                //     };

                //     let next_version = current_version.bump(increment);
                //     println!(
                //         "{}: {} -> {}",
                //         manifest.data.name().unwrap_or("<no name>"),
                //         current_version,
                //         next_version
                //     );

                //     root.set_version_req(
                //         manifest.data.name().unwrap(),
                //         next_version.clone(), // just hand over ref!
                //     );

                //     // now, do the version bump here!
                //     // we can make it interactive! ask for confirmation and
                //     // allow to select the bump type!

                //     // we compute the version bumps outside, and then just
                //     // iterate all packages and set versions and version reqs.
                //     // for this, we ideally have a notion of bumps

                //     manifest.set_version(next_version);
                // }

                // // let updated_files: Vec<PathBuf> = graph
                // //     .iter()
                // //     .enumerate()
                // //     .filter_map(|(i, m)| {
                // //         increments[i].is_some().then(|| m.path.clone())
                // //     })
                // //     .collect();

                // // enforce waiting for Cargo.lock
                // let output = std::process::Command::new("cargo")
                //     .arg("update")
                //     .arg("--workspace")
                //     .arg("--offline")
                //     // .arg("--format-version=1")
                //     .current_dir(repo.path())
                //     .output()
                //     .unwrap();

                // // 1) Stage everything
                // stage_all(repo.raw());

                // ------ we got it until here ------ ------ ------ ------ ------

                // // 2) Create the release commit
                // let message = "chore: publish\n\n...details...";
                // let oid = commit_index(repo.raw(), message).unwrap();

                // // 4) Create tag
                // let tag_name = "1.2.3"; // create tag, works as well!
                // create_tag(repo.raw(), tag_name, oid, "release").unwrap();

                // // 5) Push branch and tag
                // let branch = current_branch(repo.raw())?;
                // push_refs(
                //     repo.raw(),
                //     "origin",
                //     &[
                //         format!("HEAD:refs/heads/{branch}"),
                //         format!("refs/tags/{tag_name}"),
                //     ],
                // )?;

                // version bump!
                // ensure_clean_workdir(&repo, &[]).unwrap();

                // we KNOW all files to stage, and we should then check that
                // everything is correctly staged.

                // now, we would do the git commit

                // last but not least, update the top-level

                // bump version-reqs - how? iterate all manifests, and then
                // determine which packages we need to bump, then bump the versions

                // we might first instantiate a writer, which allows us to
                // do version bumps + dependency version bumps

                // start traversal from nodes.

                // if it's a patch, we also patch.

                // parse a config? for scopes + other settings...
                // .ctrl-z.toml

                // we practically do not need to create intermediary structs.
                // we should immediately create the right struct. now, first,
                // we determine the version bumps necessary.

                // so we should save the commit oid + message, right?

                // we don't have the notion of a workspace anymore, we just don't assign
                // commits outside of scopes, since they won't be releesd anyway
                // + we skip merges

                // for reference in repo
                //     .references()
                //     .unwrap()
                //     .flatten()
                //     .filter(Reference::is_tag)
                // {
                //     let commit = reference.commit().unwrap().unwrap();
                //     println!(
                //         "Reference: {} - {}",
                //         reference.shorthand().unwrap_or("<no name>"),
                //         commit.id()
                //     );
                // }

                // // Determine LAST version that we released = last tag.
                // let version = if let Some(last) = repo
                //     .references()
                //     .unwrap()
                //     .flatten()
                //     .filter(Reference::is_tag)
                //     .next()
                // {
                //     println!("Last reference: {:?}", last.shorthand());
                //     // try parsing this as a version...

                //     let v = semver::Version::parse(last.shorthand().unwrap())
                //         .unwrap();
                //     println!("Parsed version: {:?}", v);
                //     v
                // } else {
                //     return;
                // };

                // let proceed = Confirm::new("Do you want to proceed?")
                //     .with_default(false) // Default to NO
                //     .prompt()
                //     .unwrap();

                // if proceed {
                //     println!("Tag command executed");
                // } else {
                //     println!("Operation canceled");
                // }

                // now, based on the files in each commit, we
            }
        }
    }
}

fn ensure_clean_workdir(
    repo: &git2::Repository, allowed: &[PathBuf],
) -> Result<(), git2::Error> {
    let wd = repo
        .workdir()
        .ok_or_else(|| git2::Error::from_str("no workdir"))?;

    let allowed: HashSet<PathBuf> = allowed
        .iter()
        .map(|p| p.strip_prefix(wd).unwrap_or(p).to_path_buf())
        .collect();

    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true)
        .recurse_untracked_dirs(true)
        .include_ignored(false);

    let statuses = repo.statuses(Some(&mut opts))?;

    let offenders: Vec<_> = statuses
        .iter()
        .filter_map(|e| {
            e.path().map(|p| (Path::new(p).to_path_buf(), e.status()))
        })
        .filter(|(path, _)| !allowed.contains(path))
        .collect();

    if offenders.is_empty() {
        Ok(())
    } else {
        let detail: String = offenders
            .iter()
            .map(|(p, s)| format!("- {}: {:?}\n", p.display(), s))
            .collect();
        Err(git2::Error::from_str(&format!(
            "working tree not clean; unexpected changes:\n{}",
            detail
        )))
    }
}

fn stage_files(
    repo: &git2::Repository, files: &[PathBuf],
) -> Result<(), git2::Error> {
    let workdir = repo
        .workdir()
        .ok_or_else(|| git2::Error::from_str("no workdir"))?;

    let mut index = repo.index()?;
    for path in files {
        let rel = path
            .strip_prefix(workdir)
            .map_err(|_| git2::Error::from_str("strip_prefix"))?;
        index.add_path(rel)?;
    }
    index.write()?;
    Ok(())
}

fn stage_all(repo: &git2::Repository) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    // Stage all tracked changes and new files (respecting .gitignore)
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}

fn commit_index(
    repo: &git2::Repository, message: &str,
) -> Result<git2::Oid, git2::Error> {
    // Write tree from current index
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Prepare signatures and parent
    let sig = repo.signature()?;
    let parent = repo.head()?.peel_to_commit()?;

    // Commit
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])
}

fn current_branch(repo: &git2::Repository) -> Result<String, git2::Error> {
    let head = repo.head()?;
    head.shorthand()
        .map(|s| s.to_string())
        .ok_or_else(|| git2::Error::from_str("detached HEAD"))
}

fn push_refs(
    repo: &git2::Repository, remote_name: &str, refspecs: &[String],
) -> Result<(), git2::Error> {
    let mut remote = repo.find_remote(remote_name)?;
    let mut opts = git2::PushOptions::new();
    // Rely on system/git credential helpers by default (no callbacks)
    let refspecs: Vec<&str> = refspecs.iter().map(String::as_str).collect();
    remote.push(&refspecs, Some(&mut opts))
}

fn create_tag(
    repo: &git2::Repository, tag_name: &str, target: git2::Oid, message: &str,
) -> Result<git2::Oid, git2::Error> {
    let obj = repo.find_object(target, None)?;
    let sig = repo.signature()?;
    repo.tag(tag_name, &obj, &sig, message, false)
}

// ctrl-z-revision?

// call it project... wrap the repository

// hand over repository

fn find_packages2(repo_path: &Path) {
    // loader... <- with manifest members, we can implement a GENERAL loader!
    let root_cargo = repo_path.join("Cargo.toml");
    let cargo =
        ctrl_z_project::Project::<ctrl_z_project::Cargo>::read(root_cargo)
            .unwrap();

    // Resolver?!

    println!("Project: {:#?}", cargo);
    for member in cargo.members().flatten() {
        println!("Project.members: {:#?}", member);
    }
}

fn find_packages(repo_path: &Path) -> Graph<Manifest<Cargo>> {
    let root_cargo = repo_path.join("Cargo.toml");
    if !root_cargo.exists() {
        // throw here rather than returning nothing...
        return Graph::empty();
    }

    let root = Manifest::<Cargo>::read(&root_cargo).expect("parsing worked");

    // println!("Manifest: {:#?}", root);

    let mut packages = BTreeMap::new();

    let mut builder = Graph::builder();

    for res in root {
        let manifest = match res {
            Ok(manifest) => manifest,
            Err(err) => {
                eprintln!("Error reading manifest: {}", err);
                continue;
            }
        };

        let mut pkg_dir = manifest.path.clone();
        pkg_dir.pop(); // remove Cargo.toml, keep the folder
        packages.insert(pkg_dir, manifest.data.clone());

        // @todo: note that this only includes packages, not the top-level manifest!
        if let (Some(name), Some(version)) =
            (manifest.name(), manifest.version())
        {
            println!("{name} {version}");
            builder.add_node(manifest);
        }
    }

    let mut edges = Vec::new();
    for (n, manifest) in builder.nodes().iter().enumerate() {
        // Extract and enumerate dependencies
        let dependencies = match &manifest.data {
            Cargo::Package { dependencies, .. } => dependencies,
            Cargo::Workspace { workspace } => &workspace.dependencies,
        };

        // Add to dependencies
        for (dep_name, dependency) in dependencies {
            // if a dependency version is set, ensure that it matches!
            let m = builder
                .nodes()
                .iter() // @todo impl eq
                .position(|candidate| candidate.data.name() == Some(dep_name));

            if let Some(m) = m {
                edges.push((n, m));
            }

            match dependency.version() {
                Some(version) => println!("    - {} {}", dep_name, version),
                None => {
                    // println!("    - {} (no version specified)", dep_name)
                }
            }
        }
    }

    for (n, m) in edges {
        builder.add_edge(m, n, ()).unwrap();
    }

    // println!("Builder: {:#?}", builder);
    let graph = builder.build();
    let adj = graph.topology().outgoing();
    for n in adj {
        for &m in &adj[n] {
            println!(
                "{:?} -> {:?}",
                graph[n].data.name().unwrap(),
                graph[m].data.name().unwrap()
            );
        }
    }

    graph
}
