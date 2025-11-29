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
use ctrl_z_git::git::change::Change;
use ctrl_z_git::git::commit::Commit;
use ctrl_z_git::git::reference::Reference;
// @todo: remove the git indirection
use ctrl_z_git::git::repository::Repository;
use globset::Glob;
use inquire::Confirm;
use semver::{Version, VersionReq};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use zrx::graph::Graph;

use ctrl_z_manifest::{Cargo, Format, Manifest, PackageJson, Writer};

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

                // println!("FIND PACKAGES");
                let packages = find_packages(repo.path());
                println!(
                    "Packages: {:#?}",
                    packages.keys().collect::<Vec<_>>()
                );

                // Build scope matcher
                let mut builder = globset::GlobSetBuilder::new();
                let root = repo.path();
                // we skip the initial entry, since its a workspace and not a
                // package. we need to do this cleanly later on.
                for path in packages.keys().skip(1) {
                    let path = path.strip_prefix(root).unwrap();
                    println!("Adding scope for path: {:?}", path);
                    let pattern = path.join("**");
                    builder.add(Glob::new(pattern.to_str().unwrap()).unwrap());
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

                // fix/feat/perf/refactor/ci/build bump version

                println!(
                    "Last reference: {:?}",
                    last_ref.commit().unwrap().unwrap().id()
                );

                let last_commit = last_ref.commit().unwrap().unwrap();

                // we need all packages. for each package, we create a
                // package graph. in the end, we need to update the versions in
                // all packages and crates.

                // return;

                for commit in repo.commits().unwrap().flatten() {
                    if commit == last_commit {
                        break;
                    }
                    println!(
                        "{} - {}",
                        commit.id(),
                        commit.summary().unwrap_or("<no summary>")
                    );
                    for change in commit.changes().unwrap() {
                        let x = scopes.matches(change.path());
                        println!("{change:?} - scopes: {x:?} ");
                    }
                }

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

// pub trait Dependencies {
//     type Iter: Iterator<Item = (String, Option<VersionReq>)>;
//     fn dependencies(&self) -> Self::Iter;
// }

// impl Dependencies for Cargo {
//     type Iter = Box<dyn Iterator<Item = (String, Option<VersionReq>)>>;

//     fn dependencies(&self) -> Self::Iter {
//         match self {
//             Cargo::Package { dependencies, .. } => {
//                 let iter = dependencies
//                     .as_ref()
//                     .into_iter()
//                     .flat_map(|deps| deps.iter())
//                     .map(|(name, dep)| {
//                         let version_req = match dep {
//                             ctrl_z_manifest::manifest::format::cargo::Dependency::Version(v) => {
//                                 Some(v.clone())
//                             }
//                             _ => None,
//                         };
//                         (name.clone(), version_req)
//                     });
//                 Box::new(iter)
//             }
//             Cargo::Workspace { .. } => panic!("noes"),
//         }
//     }
// }

// call it project... wrap the repository

// hand over repository

fn find_packages(repo_path: &Path) -> BTreeMap<PathBuf, Cargo> {
    let mut root_cargo = repo_path.join("Cargo.toml");
    // println!("Repo path: {:?}", root_cargo);
    if !root_cargo.exists() {
        return BTreeMap::new();
    }

    let root = Manifest::<Cargo>::read(&root_cargo).expect("parsing worked");

    // println!("Manifest: {:#?}", root);

    let mut packages = BTreeMap::new();

    let mut builder = Graph::builder();

    for res in root.into_iter() {
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
        builder.add_edge(n, m, ()).unwrap();
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

    packages
}
