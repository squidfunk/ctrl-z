// a trait? that needs to be implemented by manifest?

use crate::{Cargo, Project, Result};
use semver::Version;
use std::collections::BTreeMap;
use std::fs;
use toml_edit::{value, DocumentMut, Item, TableLike};

pub type Versions<'a> = BTreeMap<&'a str, Version>;

pub trait Updater {
    fn update(&mut self, versions: &Versions) -> Result;
}

impl Updater for Project<Cargo> {
    fn update(&mut self, versions: &Versions) -> Result {
        // @todo: move writing in and out? that way we can safe the path! no need!
        let content = fs::read_to_string(&self.path)?;
        let mut doc = content.parse::<DocumentMut>()?;

        match &self.manifest {
            // Handle workspace manifest
            Cargo::Workspace { .. } => {
                update_workspace_dependencies(&mut doc, versions);
            }
            // Handle package manifest
            Cargo::Package { .. } => {
                update_package_version(&mut doc, versions);
                update_dependencies(&mut doc, versions);
            }
        }

        fs::write(&self.path, doc.to_string())?;
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

/// Updates `[workspace.dependencies]` with new versions.
fn update_workspace_dependencies(doc: &mut DocumentMut, versions: &Versions) {
    if let Some(table) = doc
        .get_mut("workspace")
        .and_then(|item| item.get_mut("dependencies"))
        .and_then(|item| item.as_table_like_mut())
    {
        update_dependency_table(table, versions);
    }
}

/// Updates `[package].version` with new versions.
fn update_package_version(doc: &mut DocumentMut, versions: &Versions) {
    if let Some(package) = doc
        .get_mut("package")
        .and_then(|item| item.as_table_like_mut())
    {
        if let Some(name) = package.get("name").and_then(|item| item.as_str()) {
            if let Some(version) = versions.get(name) {
                package.insert("version", value(version.to_string()));
            }
        }
    }
}

/// Updates `[dependencies]` and `[dev-dependencies]` with new versions.
fn update_dependencies(doc: &mut DocumentMut, versions: &Versions) {
    for section in ["dependencies", "dev-dependencies"] {
        if let Some(table) = doc
            .get_mut(section)
            .and_then(|item| item.as_table_like_mut())
        {
            update_dependency_table(table, versions);
        }
    }
}

// ----------------------------------------------------------------------------

/// Updates a dependency table with new versions.
fn update_dependency_table(table: &mut dyn TableLike, versions: &Versions) {
    for (name, item) in table.iter_mut() {
        if let Some(version) = versions.get(name.get()) {
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
