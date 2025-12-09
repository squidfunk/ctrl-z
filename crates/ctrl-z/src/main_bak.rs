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
use cliclack::{intro, outro, select};
use console::style;
use ctrl_z_release::Release;
// @todo: remove the git indirection
use semver::Version;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

use ctrl_z_changeset::{Increment, VersionExt};
use ctrl_z_project::{Cargo, Node, Workspace};
use ctrl_z_repository::Repository;

// pub struct ProjectConfig {
//     root: Option<String>,
// }

// // the root prpject can also be the single sink? if any?
// pub struct Config {
//     project: ProjectConfig,
// }

pub fn to_dot<T>(graph: &zrx::graph::Graph<T>) -> String
where
    T: std::fmt::Display,
{
    use std::fmt::Write;

    let mut output = String::from("digraph {\n");

    // Write nodes
    for (index, data) in graph.iter().enumerate() {
        writeln!(&mut output, "  {} [label=\"{}\"];", index, data).unwrap();
    }

    // Write edges
    let outgoing = graph.topology().outgoing();
    for node in 0..graph.len() {
        for &to in &outgoing[node] {
            writeln!(&mut output, "  {} -> {};", node, to).unwrap();
        }
    }

    output.push_str("}\n");
    output
}

// Release - provide repo...

// we need to have a "prompt tag message".

fn prompt_commit_message(
    changelog: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // Create a temporary file with template
    let mut temp = NamedTempFile::new()?;
    writeln!(temp, "chore: release")?;
    writeln!(temp, "")?;
    writeln!(temp, "# Describe the changes in this release.")?;
    // writeln!(temp, "# Edit the release message above")?;
    // writeln!(temp, "# The first line is the commit summary")?;
    // writeln!(temp, "# The changelog is included in the body")?;
    // writeln!(temp, "# Lines starting with '#' will be ignored")?;

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

// @todo: check if already installed
fn install_git_hooks() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let hook_path = Path::new(".git/hooks/commit-msg");

    // Get the current binary path
    let binary_path =
        std::env::current_exe().expect("Failed to get current executable path");

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
