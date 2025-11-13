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

use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs, io};
use clap::{Parser, Subcommand};
use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::{Styles};
use git2::{Repository};
use inquire::Confirm;
use semver::Version;
use serde::Deserialize;
use toml_edit::{Document, Item};

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
    path: PathBuf,
    version: Version,
}



fn find_packages(repo_path: &Path) -> Vec<Package> {
    let root_cargo = repo_path.join("Cargo.toml");
    if !root_cargo.exists() {
        return Vec::new();
    }

    let content = fs::read_to_string(&root_cargo).expect("Failed to read Cargo.toml");
    // let cargo: CargoToml = toml_edit::from_str(&content).expect("Failed to parse Cargo.toml");

    println!("Parsing Cargo.toml at {:?}", content);

    let mut packages = Vec::new();

    let cargo: CargoToml = toml::from_str(&content).expect("Failed to parse Cargo.toml");

    // Check if it's a workspace
    if let CargoToml::Workspace { workspace } = cargo {
        // Workspace project - find all member packages
        for member_pattern in workspace.members {
            // Simple glob support for patterns like "crates/*"
            if member_pattern.contains('*') {
                let base = member_pattern.trim_end_matches("/*");
                let base_path = repo_path.join(base);
                if let Ok(entries) = fs::read_dir(base_path) {
                    for entry in entries.flatten() {
                        let cargo_path = entry.path().join("Cargo.toml");
                        if cargo_path.exists() {
                            println!("Found package at {:?}", cargo_path);
                            // if let Some(pkg) = read_package(&cargo_path) {
                            //     packages.push(pkg);
                            // }
                        }
                    }
                }
            } else {
                let cargo_path = repo_path.join(&member_pattern).join("Cargo.toml");
                // if let Some(pkg) = read_package(&cargo_path) {
                //     packages.push(pkg);
                // }
            }
        }
    } else if let CargoToml::Package { package } = cargo {
        println!("Single package project: {:?}", package);
        // Single package project
        // packages.push(Package {
        //     name: package.name,
        //     path: repo_path.to_path_buf(),
        //     version: parse_version_tag(&package.version)
        //         .unwrap_or_else(|| Version::new(0, 0, 0)),
        // });
    }

    packages
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CargoToml {
    Package { package: PackageInfo },
    Workspace { workspace: WorkspaceInfo },
}

#[derive(Debug, Deserialize)]
struct PackageInfo {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct WorkspaceInfo {
    members: Vec<String>,
}
