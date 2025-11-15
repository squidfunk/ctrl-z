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
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

use ctrl_z_manifest::{Cargo, Manifest};

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

fn find_packages(repo_path: &Path) -> BTreeMap<PathBuf, Cargo> {
    let root_cargo = repo_path.join("Cargo.toml");
    if !root_cargo.exists() {
        return BTreeMap::new();
    }

    let manfiest = Manifest::new(&root_cargo).unwrap();
    // println!("Manifest: {:?}", manfiest);

    for m in manfiest {
        println!(">> {m:#?}")
    }

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
