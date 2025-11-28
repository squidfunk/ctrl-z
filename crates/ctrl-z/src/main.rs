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

use clap::builder::Styles;
use clap::builder::styling::{AnsiColor, Effects};
use clap::{Parser, Subcommand};
use ctrl_z_git::git::change::Change;
use ctrl_z_git::git::commit::Commit; // @todo: remove the git indirection
use ctrl_z_git::git::repository::Repository;
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
                    git2::Repository::discover(env::current_dir().unwrap())
                        .unwrap();

                let mut revwalk = repo.revwalk().unwrap();
                // @todo comment in for this to work again
                // revwalk.push_head().unwrap(); // Start from HEAD
                revwalk.set_sorting(git2::Sort::TOPOLOGICAL);

                for oid_result in revwalk {
                    match oid_result {
                        Ok(oid) => match repo.find_commit(oid) {
                            Ok(commit) => {
                                let id = commit.id();
                                let summary =
                                    commit.summary().unwrap_or("<no summary>");
                                let author = commit.author();
                                let name = author.name().unwrap_or("<no name>");
                                let email =
                                    author.email().unwrap_or("<no email>");
                                println!(
                                    "commit {} - {} <{}>",
                                    id, name, email
                                );
                                println!("    {}", summary);

                                // Iterate over all references in the repository
                                for reference in repo.references().unwrap() {
                                    if let Ok(reference) = reference {
                                        // Resolve the reference to its target
                                        if let Some(target) = reference.target()
                                        {
                                            // Check if the reference points to the given commit
                                            if target == oid {
                                                // Get the full reference name
                                                if let Some(full_name) =
                                                    reference.name()
                                                {
                                                    if full_name.starts_with(
                                                        "refs/tags/",
                                                    ) {
                                                        // It's a tag
                                                        // tags.push(full_name["refs/tags/".len()..].to_string());
                                                        println!(
                                                            "Tag: {}",
                                                            &full_name["refs/tags/".len()..]
                                                        );
                                                    } else if full_name
                                                        .starts_with(
                                                            "refs/heads/",
                                                        )
                                                    {
                                                        // It's a branch
                                                        println!(
                                                            "Branch: {}",
                                                            &full_name["refs/heads/".len()..]
                                                        );
                                                        // branches.push(full_name["refs/heads/".len()..].to_string());
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                let c = Commit::new(&repo, oid).unwrap();
                                for change in c.changes().unwrap() {
                                    println!("{:#?}", change);
                                }
                            }
                            Err(e) => eprintln!(
                                "Failed to find commit {}: {}",
                                oid, e
                            ),
                        },
                        Err(e) => eprintln!("Revwalk error: {}", e),
                    }
                }

                // get_repo();

                let repo =
                    Repository::open(env::current_dir().unwrap()).unwrap();

                for commit in repo.commits().unwrap().flatten() {
                    println!(
                        "{} - {}",
                        commit.id(),
                        commit.summary().unwrap_or("<no summary>")
                    );
                    for change in commit.changes().unwrap() {
                        println!("{:?}", change);
                    }
                }

                for reference in repo.references().unwrap().flatten() {
                    let commit = reference.commit().unwrap().unwrap();
                    println!(
                        "Reference: {} - {}",
                        reference.shorthand().unwrap_or("<no name>"),
                        commit.id()
                    );
                }

                let proceed = Confirm::new("Do you want to proceed?")
                    .with_default(false) // Default to NO
                    .prompt()
                    .unwrap();

                if proceed {
                    println!("Tag command executed");
                } else {
                    println!("Operation canceled");
                }
            }
        }
    }
}

// directly use thiserror... makes the most sense...

fn get_repo() -> io::Result<()> {
    // determine type of repository for tagging...
    let repo = git2::Repository::discover(env::current_dir()?).unwrap();
    // @todo check for uncommitted changes...

    //
    find_packages(repo.path().parent().unwrap());

    // ensure this is a Rust project!
    let cargo_toml_path = repo.path().parent().unwrap().join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Cargo.toml not found. This does not appear to be a Rust project.",
        ));
    }

    // Project :: open ... ?
    // build graph

    // Placeholder for repository retrieval logic
    println!("Retrieving repository... {:?}", repo.path());

    // we don't need to bump inherited deps. but we need to know which projects to bump

    Ok(())
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
    if !root_cargo.exists() {
        return BTreeMap::new();
    }

    let root = Manifest::<Cargo>::read(&root_cargo).expect("parsing worked");

    // println!("Manifest: {:?}", root);

    let mut builder = Graph::builder();

    for res in root.into_iter() {
        let manifest = match res {
            Ok(manifest) => manifest,
            Err(err) => {
                eprintln!("Error reading manifest: {}", err);
                continue;
            }
        };
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
    // println!("Graph: {:#?}", graph);

    // manifests form the graph nodes. next, we traverse all dependencies of the
    // manifest in order to find the packages/crates that need to be released.

    let mut packages = BTreeMap::new();

    // let cargo2 = Cargo::new(&root_cargo).expect("Failed to load Cargo.toml");
    // for x in cargo2.iter() {
    //     println!("Member: {:?}", x);
    // }

    // if let Cargo::Workspace { workspace } = &cargo2 {
    //     for member_pattern in &workspace.members {
    //         let base_path = repo_path.join(member_pattern);

    //         // Use glob to handle patterns like "crates/*"
    //         if let Ok(paths) = glob(base_path.to_str().unwrap()) {
    //             for entry in paths.flatten() {
    //                 if entry.is_dir() {
    //                     // Recursively find packages in subdirectories
    //                     packages.extend(find_packages(&entry));
    //                 } else if entry.ends_with("Cargo.toml") {
    //                     let content = fs::read_to_string(&entry)
    //                         .expect("Failed to read Cargo.toml");
    //                     let p: Cargo = toml::from_str(&content)
    //                         .expect("Failed to parse Cargo.toml");
    //                     packages.insert(entry, p);
    //                 }
    //             }
    //         }
    //     }
    // } else {
    //     packages.insert(root_cargo.clone(), cargo2);
    // }

    packages
}
