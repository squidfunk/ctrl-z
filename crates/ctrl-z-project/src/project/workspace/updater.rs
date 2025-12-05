// a trait? that needs to be implemented by manifest?

use crate::{Cargo, Project, Result};
use semver::Version;
use std::collections::BTreeMap;
use std::str::FromStr;
use toml_edit::DocumentMut;

pub type Versions<'a> = BTreeMap<&'a str, Version>;

pub trait Updater {
    fn update(&mut self, versions: &Versions) -> Result;
}

// ## Changelog

// ### Features

// - 0e83ba3 __foo-utils__ – always subtract 10

// ### Bugfixes

// - 8aa576b __foo-core__ – something was wrong

impl Updater for Project<Cargo> {
    fn update(&mut self, versions: &Versions) -> Result {
        let content = std::fs::read_to_string(&self.path)?;

        let mut doc: DocumentMut = content.parse::<DocumentMut>()?;

        match &self.manifest {
            Cargo::Workspace { .. } => {
                // Update [workspace.dependencies]
                if let Some(workspace) = doc
                    .get_mut("workspace")
                    .and_then(|item| item.as_table_mut())
                {
                    println!("Updating workspace at {:?}", self.path);
                    if let Some(dependencies) = workspace
                        .get_mut("dependencies")
                        .and_then(|item| item.as_table_mut())
                    {
                        for (dep_name, dep_item) in dependencies.iter_mut() {
                            if let Some(new_version) =
                                versions.get(dep_name.to_string().as_str())
                            {
                                println!("  Checking dependency {}", dep_name);
                                if dep_item.is_str() {
                                    println!("is a string");
                                    *dep_item = toml_edit::value(
                                        new_version.to_string(),
                                    );
                                } else if let Some(dep_table) =
                                    dep_item.as_table_like_mut()
                                {
                                    println!("is a table");
                                    dep_table.insert(
                                        "version",
                                        toml_edit::value(
                                            new_version.to_string(),
                                        ),
                                    );
                                } else {
                                    println!("nothing {:?}", dep_item);
                                }
                            }
                        }
                    }
                }
            }
            Cargo::Package { .. } => {
                // Update [package].version
                if let Some(package) =
                    doc.get_mut("package").and_then(|item| item.as_table_mut())
                {
                    if let Some(name) = package["name"].as_str() {
                        if let Some(new_version) = versions.get(name) {
                            package["version"] =
                                toml_edit::value(new_version.to_string());
                        }
                    }
                }

                if let Some(dependencies) = doc
                    .get_mut("dependencies")
                    .and_then(|item| item.as_table_mut())
                {
                    for (dep_name, dep_item) in dependencies.iter_mut() {
                        let str2 = dep_name.to_string();
                        if let Some(new_version) = versions.get(str2.as_str()) {
                            if dep_item.is_str() {
                                // Simple version string
                                *dep_item =
                                    toml_edit::value(new_version.to_string());
                            } else if let Some(dep_table) =
                                dep_item.as_table_mut()
                            {
                                // Dependency table
                                dep_table["version"] =
                                    toml_edit::value(new_version.to_string());
                            }
                        }
                    }
                }
            }
        }

        std::fs::write(&self.path, doc.to_string())?;

        // println!("Updating project at {:?}", d);

        // now, determine package

        Ok(())
    }
}

// Also impl for Workspace<T>?
