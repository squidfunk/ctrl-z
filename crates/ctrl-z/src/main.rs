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
use cliclack::log::remark;
use cliclack::{
    intro, outro, outro_note, select, set_theme, Theme, ThemeState,
};
use console::{style, Style};
// @todo: remove the git indirection
use semver::Version;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::env;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

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
                // println!("The following actions will be performed:");
                // println!("- Create a new tag");

                let repo =
                    Repository::open(env::current_dir().unwrap()).unwrap();

                // // 3) Ensure nothing else is left dirty - move this to the top!
                ensure_clean_workdir(repo.raw(), &[]).unwrap();

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

                // println!(
                //     "Last reference: {:?}",
                //     last_ref.commit().unwrap().unwrap().id()
                // );

                let last_commit = last_ref.commit().unwrap().unwrap();

                // @todo maybe changeset is created from workspace???
                // that would make scopes a private thing, which is better...
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

                let mut increments = changeset.increments().to_vec();
                let incr = increments
                    .iter()
                    .enumerate()
                    .filter_map(|(i, inc)| inc.map(|_| i))
                    .collect::<BTreeSet<_>>();

                // to_graph // to_changelog

                // changeset - function to group by scope
                // group by kind
                // or: retrieve changes for a cetain scope - the general filter
                // method... - changeset iterate revisions...

                // println!("Changeset: {:#?}", changeset);

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

                // versions...
                let mut versions = BTreeMap::<usize, &Version>::new();
                for (i, package) in workspace.packages().enumerate() {
                    if let Some(project) = workspace.get(package.1.as_str()) {
                        versions.insert(i, project.info().unwrap().1);
                    }
                }
                let versions = versions.values().collect::<Vec<_>>();

                intro("Bump versions").unwrap();

                let mut traversal = deps.graph.traverse(start);
                let incoming = deps.graph.topology().incoming();
                // we might also have an auto completing traversal?
                while let Some(node) = traversal.take() {
                    let bump = &increments[node];
                    let name = deps.graph[node].info().unwrap().0;

                    // node has no deps, we can just conitnue
                    if incoming[node].is_empty() {
                        let mut current_version = (*versions[node]).clone();
                        if let Some(x) = *bump {
                            current_version = current_version.bump(x);
                        }

                        let x = format!(
                            "{}\n{}",
                            name,
                            style(current_version).dim()
                        );
                        remark(x).unwrap();

                        // println!("  => no dependencies, accepting");
                        let _ = traversal.complete(node);
                        continue;
                    }

                    // get deps of this node
                    let mut bump_levels = BTreeSet::new();
                    for &dep in &incoming[node] {
                        let bump = &increments[dep];
                        bump_levels.insert(bump.clone());
                        // println!(
                        //     "  <- {:?} = {:?}",
                        //     deps.graph[dep].info().unwrap().0,
                        //     bump
                        // );
                    }

                    // maximum bump allowed:
                    // println!("  => max bump levels: {:?}", bump_levels);
                    // check if bump level is smaller than what we have anyway...
                    // Get the maximum bump level from dependencies
                    let max_bump = bump_levels.into_iter().flatten().max();

                    // println!("  => max bump level: {:?}", max_bump);
                    // if this is NONE, we can stop. if this is equal to the
                    // current bump, we can just take it. otherwise, we need
                    // to ask.
                    if max_bump.is_none() {
                        // println!("  => no bump needed, skipping");
                        continue;
                    }

                    if bump == &max_bump {
                        let mut current_version = (*versions[node]).clone();
                        if let Some(x) = *bump {
                            current_version = current_version.bump(x);
                        }

                        let x = format!(
                            "{}\n{}",
                            name,
                            style(current_version).dim()
                        );
                        remark(x).unwrap();
                        // println!("  => bump matches max, accepting");
                        // just accept
                    } else {
                        // what's the suggested bump? minor!
                        // println!("  => current version: {}", current_version);

                        // in case major minor patch are all the same, only
                        // show the PATCH option.

                        // auto select if there's only one possible version!
                        // but no, we can also skip...

                        // what's the suggested bump? minor!
                        let current_version = (*versions[node]).clone();

                        // Compute all possible versions
                        let patch_version =
                            current_version.bump(Increment::Patch);
                        let minor_version =
                            current_version.bump(Increment::Minor);
                        let major_version =
                            current_version.bump(Increment::Major);

                        // Build select options, skipping duplicates (e.g., in 0.0.x)
                        let mut select_builder =
                            select(name).item(None, current_version, "current");

                        // Always add patch
                        select_builder = select_builder.item(
                            Some(Increment::Patch),
                            patch_version.clone(),
                            "patch",
                        );

                        // Only add minor if different from patch
                        if minor_version != patch_version {
                            select_builder = select_builder.item(
                                Some(Increment::Minor),
                                minor_version.clone(),
                                "minor",
                            );
                        }

                        // Only add major if different from minor
                        if major_version != minor_version {
                            select_builder = select_builder.item(
                                Some(Increment::Major),
                                major_version,
                                "major",
                            );
                        }

                        let selected = select_builder
                            .initial_value(Some(Increment::Patch)) // or skip?
                            .interact()
                            .unwrap(); // io result!

                        increments[node] = selected;
                    }

                    // if node in sources, just skip!
                    let _ = traversal.complete(node);
                }

                outro("Completed").unwrap();

                println!("increments {:?}", increments);

                // bumps - go through _all_ of them... start at sources...
                // if the downstream package is not affected - skip the entire thing!

                // walk through all packages here in TOPO order.

                let mut new_versions = BTreeMap::new();
                for (node, incr) in increments.into_iter().enumerate() {
                    if let Some(increment) = incr {
                        let name =
                            deps.graph[node].info().unwrap().0.to_string();
                        let version = (*versions[node]).clone();
                        // compute the actual bumps
                        new_versions.insert(name, version.bump(increment));
                    }
                }

                println!("new versions {:?}", new_versions);

                for project in &mut workspace {
                    project
                        .update(
                            &new_versions
                                .iter()
                                .map(|(k, v)| (k.as_str(), v.clone()))
                                .collect(),
                        )
                        .unwrap();
                }

                // enforce waiting for Cargo.lock
                let output = std::process::Command::new("cargo")
                    .arg("update")
                    .arg("--workspace")
                    .arg("--offline")
                    // .arg("--format-version=1")
                    .current_dir(repo.path())
                    .output()
                    .unwrap();

                // // 1) Stage everything
                stage_all(repo.raw());

                // ------ we got it until here ------ ------ ------ ------ ------

                let changelog = changeset.to_changelog().to_string();

                let message = prompt_commit_message(&changelog).unwrap();
                let oid = commit_index(repo.raw(), &message).unwrap();

                // // 2) Create the release commit
                // let message = "chore: publish\n\n...details...";
                // let oid = commit_index(repo.raw(), message).unwrap();

                // // 4) Create tag
                let tag_name = "v1.2.3"; // create tag, works as well!
                create_tag(repo.raw(), tag_name, oid, "").unwrap();

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

fn create_tag_interactive(
    repo: &git2::Repository, tag_name: &str, target: git2::Oid,
) -> Result<git2::Oid, Box<dyn std::error::Error>> {
    let message = prompt_tag_message(tag_name)?;
    let obj = repo.find_object(target, None)?;
    let sig = repo.signature()?;
    Ok(repo.tag(tag_name, &obj, &sig, &message, false)?)
}

fn prompt_tag_message(
    tag_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create a temporary file with template
    let mut temp = NamedTempFile::new()?;
    writeln!(
        temp,
        "# Tag message for {}\n# Lines starting with '#' will be ignored",
        tag_name
    )?;

    // Get editor from environment or use default
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string());

    // Open editor
    let status = std::process::Command::new(&editor)
        .arg(temp.path())
        .status()?;

    if !status.success() {
        return Err("Editor exited with non-zero status".into());
    }

    // Read the message back
    let content = std::fs::read_to_string(temp.path())?;

    // Filter out comment lines and trim
    let message: String = content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if message.is_empty() {
        return Err("Empty tag message".into());
    }

    Ok(message)
}

fn prompt_commit_message(
    changelog: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create a temporary file with template
    let mut temp = NamedTempFile::new()?;
    writeln!(temp, "chore: release\n")?;
    writeln!(temp, "{}\n", changelog)?;
    writeln!(temp, "# Edit the release message above")?;
    writeln!(temp, "# The first line is the commit summary")?;
    writeln!(temp, "# The changelog is included in the body")?;
    writeln!(temp, "# Lines starting with '#' will be ignored")?;

    // Get editor from environment or use default
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vim".to_string());

    // Open editor
    let status = std::process::Command::new(&editor)
        .arg(temp.path())
        .status()?;

    if !status.success() {
        return Err("Editor exited with non-zero status".into());
    }

    // Read the message back
    let content = std::fs::read_to_string(temp.path())?;

    // Filter out comment lines and trim
    let message: String = content
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string();

    if message.is_empty() {
        return Err("Empty commit message".into());
    }

    Ok(message)
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
