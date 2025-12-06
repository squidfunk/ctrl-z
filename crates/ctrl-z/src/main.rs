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
use ctrl_z_project::workspace::updater::Updatable;
// @todo: remove the git indirection
use inquire::Select;
use semver::Version;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::path::{Path, PathBuf};

use ctrl_z_changeset::{Changeset, Increment, Scopes, VersionExt};
use ctrl_z_project::{Cargo, Node, Workspace};
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
    /// Git hooks
    Hook {
        #[command(subcommand)]
        hook_type: HookCommands,
    },
}

#[derive(Subcommand)]
enum HookCommands {
    /// Validate commit message format
    CommitMsg {
        /// Path to commit message file
        #[arg(default_value = ".git/COMMIT_EDITMSG")]
        message_file: PathBuf,
    },
    Install,
}

#[allow(warnings)]
pub fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Hook { hook_type } => match hook_type {
            HookCommands::CommitMsg { message_file } => {
                handle_commit_msg_hook(&message_file);
            }
            HookCommands::Install => {
                install_git_hooks();
            }
        },
        Commands::Tag { dry_run } => {
            if dry_run {
                println!("Dry run: no changes will be made");
            } else {
                println!("The following actions will be performed:");
                println!("- Create a new tag");

                let repo =
                    Repository::open(env::current_dir().unwrap()).unwrap();

                let path = repo.path().join("package.json");
                if path.exists() {
                    let mut workspace = Workspace::<Node>::read(path).unwrap();

                    let deps = workspace.dependents().unwrap();

                    // print as .dot!

                    let mut traversal =
                        deps.graph.traverse(deps.graph.sources());
                    // we might also have an auto completing traversal?
                    while let Some(node) = traversal.take() {
                        println!(
                            " - Process package: {:?}",
                            deps.graph[node].info()
                        );
                        traversal.complete(node).unwrap();
                    }

                    // let versions = BTreeMap::from_iter([
                    //     (
                    //         "@zensical/disco-engine",
                    //         Version::parse("0.1.0").unwrap(),
                    //     ),
                    //     ("@zensical/disco", Version::parse("0.0.1").unwrap()),
                    // ]);

                    // for project in &mut workspace {
                    //     project.update(&versions).unwrap();
                    // }
                    return;
                }

                //
                let path = repo.path().join("Cargo.toml");
                let mut workspace = Workspace::<Cargo>::read(path).unwrap();

                let scopes = Scopes::try_from(&workspace).unwrap();

                // @todo remove when we can build the graph and scope from it.
                // in the graph, we now determine all actually versioned packages
                // and their deps
                // let graph = create_graph(&projects);

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

                let mut changeset = Changeset::new(scopes);
                let commits = repo
                    .commits()
                    .unwrap()
                    .flatten()
                    .take_while(|commit| commit != &last_commit);

                changeset.extend(commits).unwrap();

                // changeset: collect for scope!

                println!("{}", changeset.to_changelog());

                // we should impl a partial revision iterator?

                let increments = changeset.increments().to_vec();
                let incr = increments
                    .iter()
                    .enumerate()
                    .filter_map(|(i, inc)| inc.map(|_| i))
                    .collect::<BTreeSet<_>>();

                // to_graph // to_changelog

                // @todo: directly map the scopes in Scopes! We can then generate
                // everything from them immediately...

                // changeset - function to group by scope
                // group by kind
                // or: retrieve changes for a cetain scope - the general filter
                // method... - changeset iterate revisions...

                println!("Changeset: {:#?}", changeset);

                // so from the bumped packages, we must identify the sources.
                // thus, we can just determine ALL sources, and then intersect
                // those with those that were bumped. then we use this for
                // traversal, since we don't need to iterate all of them.

                let deps = workspace.dependents().unwrap();
                let mut sources: BTreeSet<usize> =
                    deps.graph.sources().collect();

                // println!("sources {:?}", sources);
                // println!("incr {:?}", incr);
                let start = sources
                    .intersection(&incr)
                    .copied()
                    .collect::<HashSet<_>>();
                // println!("start {:?}", start);

                // inherit bumps = re-export package

                let mut traversal = deps.graph.traverse(start);
                let incoming = deps.graph.topology().incoming();
                // we might also have an auto completing traversal?
                while let Some(node) = traversal.take() {
                    let bump = &increments[node];
                    println!(
                        "{:?} = {:?}",
                        deps.graph[node].info().unwrap().0,
                        bump
                    );

                    // node has no deps, we can just conitnue
                    if incoming[node].is_empty() {
                        println!("  => no dependencies, accepting");
                        let _ = traversal.complete(node);
                        continue;
                    }

                    // get deps of this node
                    let mut bump_levels = BTreeSet::new();
                    for &dep in &incoming[node] {
                        let bump = &increments[dep];
                        bump_levels.insert(bump.clone());
                        println!(
                            "  <- {:?} = {:?}",
                            deps.graph[dep].info().unwrap().0,
                            bump
                        );
                    }

                    // maximum bump allowed:
                    println!("  => max bump levels: {:?}", bump_levels);
                    // check if bump level is smaller than what we have anyway...
                    // Get the maximum bump level from dependencies
                    let max_bump = bump_levels.into_iter().flatten().max();

                    println!("  => max bump level: {:?}", max_bump);
                    // if this is NONE, we can stop. if this is equal to the
                    // current bump, we can just take it. otherwise, we need
                    // to ask.
                    if max_bump.is_none() {
                        println!("  => no bump needed, skipping");
                        continue;
                    }

                    if bump == &max_bump {
                        println!("  => bump matches max, accepting");
                        // just accept
                    } else {
                        // here, we need to prompt...
                        println!("prompt the user!");
                        // // prompt
                        // let x = prompt_increment(
                        //     deps.graph[node]
                        //         .manifest
                        //         .name()
                        //         .unwrap_or("<no name>"),
                        //     deps.graph[node]
                        //         .manifest
                        //         .version()
                        //         .expect("versioned package"),
                        //     max_bump,
                        //     &[],
                        // );

                        // match x {
                        //     Ok(x) => {
                        //         println!(
                        //             "  => User selected increment: {:?}",
                        //             x
                        //         );
                        //         // set increment here!
                        //     }
                        //     Err(err) => {
                        //         eprintln!(
                        //             "  => Error prompting for increment: {}",
                        //             err
                        //         );
                        //         break;
                        //     }
                        // }
                    }

                    // if node in sources, just skip!
                    let _ = traversal.complete(node);
                }

                // bumps - go through _all_ of them... start at sources...
                // if the downstream package is not affected - skip the entire thing!

                // walk through all packages here in TOPO order.

                // println!(
                //     "Increments needed for scopes at indexes: {:#?}",
                //     incr
                // );

                // // now, determine actual version bumps!
                // let versions = BTreeMap::from_iter([
                //     ("foo-utils", Version::parse("0.1.0").unwrap()),
                //     ("foo-core", Version::parse("0.0.2").unwrap()),
                // ]);

                // for project in &mut workspace {
                //     project.update(&versions).unwrap();
                // }

                // how go get version for scope? workspace.get?
                // now, traverse from all sources?

                // Writer <- this should take a workspace, and then write all

                // move prompt outside - provide a callback function that receives
                // the recommended version bump and can return one as well.

                // // can we also impl an iterator over the traversal that auto-completes?
                // let mut traversal = graph.traverse(incr);
                // let inc = graph.topology().incoming();
                // while let Some(node) = traversal.take() {
                //     println!("Traversed node: {:?} - {:?}", node, &inc[node]);

                //     // we just need to list:
                //     // - the dependent package name and its version
                //     // - all commits related to the package (= scope)

                //     let x = prompt_increment(
                //         graph[node].manifest.name().unwrap(),
                //         graph[node]
                //             .manifest
                //             .version()
                //             .expect("versioned package"),
                //         increments[node],
                //         &[],
                //     );

                //     match x {
                //         Ok(x) => {
                //             println!("User selected increment: {:?}", x);
                //         }
                //         Err(err) => {
                //             eprintln!("Error prompting for increment: {}", err);
                //             break;
                //         }
                //     }

                //     traversal.complete(node);
                // }

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

// fn find_packages2(
//     repo_path: &Path,
// ) -> Result<BTreeMap<PathBuf, Project<Cargo>>, Error> {
//     // loader... <- with manifest members, we can implement a GENERAL loader!
//     let root_cargo = repo_path.join("Cargo.toml");
//     let project = Project::<Cargo>::read(root_cargo)?;
//     project
//         .into_iter()
//         .map(|res| {
//             res.map(|member| {
//                 let dir = member
//                     .path
//                     .parent()
//                     .expect("manifest has parent")
//                     // .strip_prefix(repo_path)
//                     // .expect("strip_prefix")
//                     .to_path_buf();
//                 (dir, member)
//             })
//         })
//         .collect()
// }

// // we should create a graph of scopes, so the scopes are the nodes!
// // we should check what we need from the project and only use that, and then
// // from the project create the scopes...?
// fn create_graph(
//     projects: &BTreeMap<PathBuf, Project<Cargo>>,
// ) -> Graph<&Project<Cargo>> {
//     // so this is  a graph fo refs...
//     let mut builder = Graph::builder::<()>();
//     for (path, project) in projects {
//         if project.manifest.version().is_none() {
//             continue;
//         }
//         builder.add_node(project);
//     }

//     // add edges...
//     let mut edges = Vec::new();
//     for (n, manifest) in builder.nodes().iter().enumerate() {
//         // Extract and enumerate dependencies
//         let dependencies = match &manifest.manifest {
//             Cargo::Package { dependencies, .. } => dependencies,
//             Cargo::Workspace { workspace } => &workspace.dependencies,
//         };

//         // Add to dependencies
//         for (dep_name, dependency) in dependencies {
//             // if a dependency version is set, ensure that it matches!
//             let m = builder
//                 .nodes()
//                 .iter() // @todo impl eq
//                 .position(|candidate| {
//                     candidate.manifest.name() == Some(dep_name)
//                 });

//             if let Some(m) = m {
//                 edges.push((n, m));
//             }
//         }
//     }

//     for (n, m) in edges {
//         builder.add_edge(m, n, ()).unwrap();
//     }

//     builder.build()
//     // Here, we collect all versions
// }

// /// Updates a package.json manifest string with new versions.
// fn update_npm_manifest(content: &str, versions: &Versions) -> Result<String> {
//     let mut doc: Value = serde_json::from_str(content)?;

//     // Update package version if name matches
//     if let Some(obj) = doc.as_object_mut() {
//         if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
//             if let Some(new_version) = versions.get(name) {
//                 obj.insert("version".to_string(), Value::String(new_version.to_string()));
//             }
//         }

//         // Update dependencies sections
//         for section in ["dependencies", "devDependencies", "peerDependencies"] {
//             if let Some(deps) = obj.get_mut(section).and_then(|d| d.as_object_mut()) {
//                 update_npm_dependency_table(deps, versions);
//             }
//         }
//     }

//     // Pretty-print with 2-space indentation
//     Ok(serde_json::to_string_pretty(&doc)?)
// }

// fn update_npm_dependency_table(
//     deps: &mut serde_json::Map<String, Value>,
//     versions: &Versions,
// ) {
//     for (dep_name, dep_value) in deps.iter_mut() {
//         if let Some(new_version) = versions.get(dep_name.as_str()) {
//             *dep_value = Value::String(new_version.to_string());
//         }
//     }
// }

fn handle_commit_msg_hook(message_file: &Path) {
    use ctrl_z_changeset::Change;
    use std::fs;
    use std::str::FromStr;

    // Read commit message
    let content = match fs::read_to_string(message_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading commit message: {}", e);
            std::process::exit(1);
        }
    };

    // Get first non-comment line
    let message = content
        .lines()
        .find(|line| {
            !line.trim().is_empty() && !line.trim_start().starts_with('#')
        })
        .unwrap_or("");

    // @todo also check for refs... can we use inquire here?

    // Validate using Change parser
    match Change::from_str(message) {
        Ok(change) => {
            println!("✓ Valid commit message: {:?}", change.kind());
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("✗ Invalid commit message format");
            eprintln!("  {}", e);
            eprintln!("\nExpected format:");
            eprintln!("  <type>[!]: <description>");
            eprintln!("\nValid types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert");
            eprintln!("Add '!' for breaking changes");
            std::process::exit(1);
        }
    }
}

// @todo: check if already installed
fn install_git_hooks() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let hook_path = Path::new(".git/hooks/commit-msg");

    // Get the current binary path
    let binary_path =
        env::current_exe().expect("Failed to get current executable path");

    let hook_content = format!(
        "#!/bin/sh\n{} hook commit-msg \"$1\"\n",
        binary_path.display()
    );

    // Write hook file
    fs::write(hook_path, hook_content)
        .expect("Failed to write commit-msg hook");

    // Make executable (Unix only)
    #[cfg(unix)]
    {
        let mut perms = fs::metadata(hook_path)
            .expect("Failed to get hook metadata")
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(hook_path, perms)
            .expect("Failed to set hook permissions");
    }

    println!("✓ Installed commit-msg hook at {}", hook_path.display());
}
