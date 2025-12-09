// a trait? that needs to be implemented by manifest?

use crate::{Cargo, Manifest, Node, Project, Result};
use semver::Version;
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use toml_edit::{value, DocumentMut, Item, TableLike};

// add to workspace as well! - compute all versions, then bump everything.

impl<T> Project<T>
where
    T: Manifest,
{
    pub fn update(&mut self, versions: &Versions) -> Result
    where
        T: Updatable,
    {
        let content = fs::read_to_string(&self.path)?;
        let updated = T::update(&content, versions)?;
        fs::write(&self.path, updated)?;
        Ok(())
    }
}

// versions are here... first, we refactor this stuff here...
pub type Versions<'a> = BTreeMap<&'a str, Version>;

pub trait Updatable {
    fn update<S>(content: S, versions: &Versions) -> Result<String>
    where
        S: AsRef<str>;
}

impl Updatable for Cargo {
    fn update<S>(content: S, versions: &Versions) -> Result<String>
    where
        S: AsRef<str>,
    {
        let content = content.as_ref();
        // @todo: move writing in and out? that way we can safe the path! no need!
        // we might just parse this from string? again?
        let mut doc = content.parse::<DocumentMut>()?;
        update_workspace_dependencies(&mut doc, versions);
        update_package_version(&mut doc, versions);
        update_dependencies(&mut doc, versions);

        // we need to wait for the cargo toml to catch up.

        // // enforce waiting for Cargo.lock
        let output = std::process::Command::new("cargo")
            .arg("update")
            .arg("--workspace")
            .arg("--offline")
            // .arg("--format-version=1")
            // .current_dir(repo.path())
            .output()
            .unwrap();

        Ok(doc.to_string())
    }
}

impl Updatable for Node {
    fn update<S>(content: S, versions: &Versions) -> Result<String>
    where
        S: AsRef<str>,
    {
        // @todo: move writing in and out? that way we can safe the path! no need!
        // we might just parse this from string? again?
        let content = content.as_ref();
        let mut doc = serde_json::from_str::<Value>(content)?;
        // update_workspace_dependencies(&mut doc, versions);
        // NPM_update_package_version(&mut doc, versions);

        // Update package version if name matches
        if let Some(obj) = doc.as_object_mut() {
            if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                if let Some(new_version) = versions.get(name) {
                    obj.insert(
                        "version".to_string(),
                        Value::String(new_version.to_string()),
                    );
                }
            }

            // Update dependencies sections
            for section in
                ["dependencies", "devDependencies", "peerDependencies"]
            {
                if let Some(deps) =
                    obj.get_mut(section).and_then(|value| value.as_object_mut())
                {
                    update_npm_dependency_table(deps, versions);
                }
            }
        }

        let mut content = serde_json::to_string_pretty(&doc)?;
        content.push('\n');
        Ok(content)
    }
}

fn update_npm_dependency_table(
    map: &mut serde_json::Map<String, Value>, versions: &Versions,
) {
    for (name, value) in map.iter_mut() {
        if let Some(version) = versions.get(name.as_str()) {
            *value = Value::String(format!("^{version}"));
        }
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
