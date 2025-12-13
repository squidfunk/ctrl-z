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

//! @todo tbd

use semver::Version;
use toml_edit::{value, DocumentMut, Item, TableLike};

use crate::project::manifest::cargo::Cargo;
use crate::project::Result;

use super::{Writable, Writer};

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Writable for Writer<'_, Cargo> {
    fn write<S>(&self, input: S) -> Result<String>
    where
        S: AsRef<str>,
    {
        let content = input.as_ref();
        // @todo: move writing in and out? that way we can safe the path! no need!
        // we might just parse this from string? again?
        let mut doc = content.parse::<DocumentMut>()?;
        update_workspace_dependencies(&mut doc, self);
        update_package_version(&mut doc, self);
        update_dependencies(&mut doc, self);

        // we need to wait for the cargo toml to catch up.

        // enforce waiting for Cargo.lock
        let output = std::process::Command::new("cargo")
            .arg("update")
            .arg("--workspace")
            .arg("--offline")
            .output()
            .unwrap();

        Ok(doc.to_string())
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Updates `[workspace.dependencies]` with new versions.
fn update_workspace_dependencies(
    doc: &mut DocumentMut, writer: &Writer<Cargo>,
) {
    if let Some(table) = doc
        .get_mut("workspace")
        .and_then(|item| item.get_mut("dependencies"))
        .and_then(|item| item.as_table_like_mut())
    {
        update_dependency_table(table, writer);
    }
}

/// Updates `[package].version` with new versions.
fn update_package_version(doc: &mut DocumentMut, writer: &Writer<Cargo>) {
    if let Some(package) = doc
        .get_mut("package")
        .and_then(|item| item.as_table_like_mut())
    {
        if let Some(name) = package.get("name").and_then(|item| item.as_str()) {
            if let Some(version) = writer.items.get(name) {
                package.insert("version", value(version.to_string()));
            }
        }
    }
}

/// Updates `[dependencies]` and `[dev-dependencies]` with new versions.
fn update_dependencies(doc: &mut DocumentMut, writer: &Writer<Cargo>) {
    for section in ["dependencies", "dev-dependencies"] {
        if let Some(table) = doc
            .get_mut(section)
            .and_then(|item| item.as_table_like_mut())
        {
            update_dependency_table(table, writer);
        }
    }
}

// ----------------------------------------------------------------------------

/// Updates a dependency table with new versions.
fn update_dependency_table(table: &mut dyn TableLike, writer: &Writer<Cargo>) {
    for (name, item) in table.iter_mut() {
        if let Some(version) = writer.items.get(name.get()) {
            update_dependency(item, version);
        }
    }
}

/// Updates a dependency with a new version.
fn update_dependency(item: &mut Item, version: &Version) {
    if let Some(table) = item.as_table_like() {
        if let Some(workspace) = table.get("workspace") {
            // Skip if dependency inherits from workspace
            if workspace.as_bool() == Some(true) {
                return;
            }
        }
    }

    // Update simple version string: `foo = "1.0.0"`
    if item.is_str() {
        *item = value(version.to_string());

    // Update inline table: `foo = { version = "1.0.0" }`
    } else if let Some(table) = item.as_table_like_mut() {
        table.insert("version", value(version.to_string()));
    }
}
