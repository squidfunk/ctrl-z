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
use git2::Repository;
use inquire::Confirm;
use semver::Version;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs, io};
use toml_edit::{Document, Item};

use ctrl_z_tag::manifest::Cargo;

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

                get_repo();

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
    let repo = Repository::discover(env::current_dir()?).unwrap();
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

    //

    // Placeholder for repository retrieval logic
    println!("Retrieving repository... {:?}", repo.path());

    Ok(())
}

#[derive(Debug)]
struct Package {
    name: String,
    version: Version,
    // dependencies <- we must analyze dependencies as well...
}

fn find_packages(repo_path: &Path) -> BTreeMap<PathBuf, CargoToml> {
    let root_cargo = repo_path.join("Cargo.toml");
    if !root_cargo.exists() {
        return BTreeMap::new();
    }

    let content =
        fs::read_to_string(&root_cargo).expect("Failed to read Cargo.toml");
    let mut packages = BTreeMap::new();

    let cargo: CargoToml =
        toml::from_str(&content).expect("Failed to parse Cargo.toml");

    let cargo2 = Cargo::new(&root_cargo).expect("Failed to load Cargo.toml");
    println!("Cargo2: {:#?}", cargo2);
    // @todo: next, move resolution into Cargo module...
    // create a map/graph of dependencies.

    // Check if it's a workspace
    // derive scopes from workspace members or from config file...
    if let CargoToml::Workspace { workspace } = &cargo {
        for member_pattern in &workspace.members {
            let base_path =
                repo_path.join(member_pattern.trim_end_matches("/*"));

            if let Ok(entries) = fs::read_dir(&base_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        // Recursively find packages in subdirectories
                        packages.extend(find_packages(&path));
                    } else if path.ends_with("Cargo.toml") {
                        let content = fs::read_to_string(&path)
                            .expect("Failed to read Cargo.toml");
                        let p: CargoToml = toml::from_str(&content)
                            .expect("Failed to parse Cargo.toml");
                        packages.insert(path, p);
                    }
                }
            }
        }
    } else {
        packages.insert(root_cargo.clone(), cargo);
    }

    println!("Found packages: {:#?}", packages);

    // packages = scopes in a rust crate!

    packages
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CargoToml {
    Package {
        package: PackageInfo,
        dependencies: Option<BTreeMap<String, Dependency>>,
    },
    Workspace {
        workspace: WorkspaceInfo,
    },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Dependency {
    Simple(String),
    Detailed {
        version: Option<String>,
        workspace: Option<bool>,
    },
}

#[derive(Debug, Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct WorkspaceInfo {
    members: Vec<String>,
    dependencies: Option<BTreeMap<String, Dependency>>,
}
