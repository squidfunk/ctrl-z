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

//! todo

use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::{Parser, Subcommand};
// @todo: remove the git indirection
use inquire::Select;
use semver::Version;
use std::collections::{BTreeMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};
use zrx::graph::Graph;

use ctrl_z_changeset::{Changeset, Increment, Scope, VersionExt};
use ctrl_z_project::{Cargo, Error, Manifest as _, Project, Workspace};
use ctrl_z_repository::Reference;
use ctrl_z_repository::Repository;

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

                //
                let path = repo.path().join("Cargo.toml");
                let workspace = Workspace::<Cargo>::read(path).unwrap();

                // Scopr try_from
                let scope = Scope::try_from(&workspace).unwrap();

                // @todo remove when we can build the graph and scope from it.
                let projects = find_packages2(repo.path()).unwrap();
                // in the graph, we now determine all actually versioned packages
                // and their deps
                let graph = create_graph(&projects);

                // Project Collection?

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
                let mut changeset = Changeset::new(scope);
                let commits = repo
                    .commits()
                    .unwrap()
                    .flatten()
                    .take_while(|commit| commit != &last_commit);

                changeset.extend(commits).unwrap();

                // changeset: collect for scope!

                println!("Changeset: {:#?}", changeset);
                println!("{}", changeset.to_changelog());

                let mut increments = changeset.increments().to_vec();
                let incr = increments
                    .iter()
                    .enumerate()
                    .filter_map(|(i, inc)| inc.map(|_| i))
                    .collect::<Vec<_>>();

                // to_graph // to_changelog

                // @todo: directly map the scopes in Scopes! We can then generate
                // everything from them immediately...

                // changeset - function to group by scope
                // group by kind
                // or: retrieve changes for a cetain scope - the general filter
                // method... - changeset iterate revisions...

                println!(
                    "Increments needed for scopes at indexes: {:#?}",
                    incr
                );

                // move prompt outside - provide a callback function that receives
                // the recommended version bump and can return one as well.

                // can we also impl an iterator over the traversal that auto-completes?
                let mut traversal = graph.traverse(incr);
                let inc = graph.topology().incoming();
                while let Some(node) = traversal.take() {
                    println!("Traversed node: {:?} - {:?}", node, &inc[node]);

                    let x = prompt_increment(
                        graph[node].manifest.name().unwrap(),
                        graph[node]
                            .manifest
                            .version()
                            .expect("versioned package"),
                        increments[node],
                        &[],
                    );

                    match x {
                        Ok(x) => {
                            println!("User selected increment: {:?}", x);
                        }
                        Err(err) => {
                            eprintln!("Error prompting for increment: {}", err);
                            break;
                        }
                    }

                    traversal.complete(node);
                }

                // traversal!

                // get node indexes

                // graph.traverse(initial)

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

fn prompt_increment(
    package_name: &str,
    current_version: &Version,
    suggested: Option<Increment>,
    changes: &[(String, String)], // (hash, message)
) -> Result<Option<Increment>, inquire::InquireError> {
    println!("\n{} (currently {})", package_name, current_version);

    if let Some(inc) = suggested {
        let next = current_version.clone().bump(inc);
        println!("  Suggested: {:?} → {}\n", inc, next);
    }

    if !changes.is_empty() {
        println!("  Changes:");
        for (hash, msg) in changes {
            println!("  ✓ {} ({})", msg, &hash[..7]);
        }
        println!();
    }

    let options = vec![
        ("Accept", suggested),
        ("Patch", Some(Increment::Patch)),
        ("Minor", Some(Increment::Minor)),
        ("Major", Some(Increment::Major)),
        ("Skip", None),
    ];

    let choice =
        Select::new("Select bump:", options.iter().map(|x| x.0).collect())
            .prompt()?;

    Ok(options.into_iter().find(|x| x.0 == choice).unwrap().1)
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

fn find_packages2(
    repo_path: &Path,
) -> Result<BTreeMap<PathBuf, Project<Cargo>>, Error> {
    // loader... <- with manifest members, we can implement a GENERAL loader!
    let root_cargo = repo_path.join("Cargo.toml");
    let project = Project::<Cargo>::read(root_cargo)?;
    project
        .into_iter()
        .map(|res| {
            res.map(|member| {
                let dir = member
                    .path
                    .parent()
                    .expect("manifest has parent")
                    // .strip_prefix(repo_path)
                    // .expect("strip_prefix")
                    .to_path_buf();
                (dir, member)
            })
        })
        .collect()
}

// we should create a graph of scopes, so the scopes are the nodes!
// we should check what we need from the project and only use that, and then
// from the project create the scopes...?
fn create_graph(
    projects: &BTreeMap<PathBuf, Project<Cargo>>,
) -> Graph<&Project<Cargo>> {
    // so this is  a graph fo refs...
    let mut builder = Graph::builder::<()>();
    for (path, project) in projects {
        if project.manifest.version().is_none() {
            continue;
        }
        builder.add_node(project);
    }

    // add edges...
    let mut edges = Vec::new();
    for (n, manifest) in builder.nodes().iter().enumerate() {
        // Extract and enumerate dependencies
        let dependencies = match &manifest.manifest {
            Cargo::Package { dependencies, .. } => dependencies,
            Cargo::Workspace { workspace } => &workspace.dependencies,
        };

        // Add to dependencies
        for (dep_name, dependency) in dependencies {
            // if a dependency version is set, ensure that it matches!
            let m = builder
                .nodes()
                .iter() // @todo impl eq
                .position(|candidate| {
                    candidate.manifest.name() == Some(dep_name)
                });

            if let Some(m) = m {
                edges.push((n, m));
            }
        }
    }

    for (n, m) in edges {
        builder.add_edge(m, n, ()).unwrap();
    }

    builder.build()
    // Here, we collect all versions
}
