// a trait? that needs to be implemented by manifest?

use crate::{Cargo, Project, Result};
use semver::Version;
use std::collections::BTreeMap;
use std::str::FromStr;
use toml_edit::{DocumentMut, Item, TableLike};

pub type Versions<'a> = BTreeMap<&'a str, Version>;

pub trait Updater {
    fn update(&mut self, versions: &Versions) -> Result;
}

impl Updater for Project<Cargo> {
    fn update(&mut self, versions: &Versions) -> Result {
        let content = std::fs::read_to_string(&self.path)?;
        let mut doc = content.parse::<DocumentMut>()?;

        match &self.manifest {
            Cargo::Workspace { .. } => {
                update_workspace_dependencies(&mut doc, versions);
            }
            Cargo::Package { .. } => {
                update_package_version(&mut doc, versions);
                update_dependencies(&mut doc, "dependencies", versions);
            }
        }

        std::fs::write(&self.path, doc.to_string())?;
        Ok(())
    }
}

/// Updates [workspace.dependencies] with new versions.
fn update_workspace_dependencies(doc: &mut DocumentMut, versions: &Versions) {
    if let Some(deps) = doc
        .get_mut("workspace")
        .and_then(|w| w.get_mut("dependencies"))
        .and_then(|d| d.as_table_like_mut())
    {
        update_dependency_table(deps, versions);
    }
}

/// Updates [package].version if the package name matches.
fn update_package_version(doc: &mut DocumentMut, versions: &Versions) {
    if let Some(package) = doc.get_mut("package").and_then(|p| p.as_table_mut())
    {
        if let Some(name) = package.get("name").and_then(|n| n.as_str()) {
            if let Some(new_version) = versions.get(name) {
                package["version"] = toml_edit::value(new_version.to_string());
            }
        }
    }
}

/// Updates a dependency section (e.g., [dependencies], [dev-dependencies]).
fn update_dependencies(
    doc: &mut DocumentMut, section: &str, versions: &Versions,
) {
    if let Some(deps) = doc.get_mut(section).and_then(|d| d.as_table_like_mut())
    {
        update_dependency_table(deps, versions);
    }
}

/// Updates all entries in a dependency table (handles both string and inline table).
fn update_dependency_table(deps: &mut dyn TableLike, versions: &Versions) {
    for (dep_name, dep_item) in deps.iter_mut() {
        if let Some(new_version) = versions.get(dep_name.get()) {
            update_dependency_item(dep_item, new_version);
        }
    }
}

/// Updates a single dependency item (string or inline table).
fn update_dependency_item(item: &mut Item, version: &Version) {
    // Skip if dependency uses workspace = true
    if let Some(table) = item.as_table_like() {
        if let Some(workspace_val) = table.get("workspace") {
            if workspace_val.as_bool() == Some(true) {
                return; // Don't update workspace-inherited dependencies
            }
        }
    }

    let version_str = version.to_string();

    if item.is_str() {
        // Simple version string: foo = "1.0.0"
        *item = toml_edit::value(version_str);
    } else if let Some(table) = item.as_table_like_mut() {
        // Inline table: foo = { version = "1.0.0", features = [...] }
        table.insert("version", toml_edit::value(version_str));
    }
}
