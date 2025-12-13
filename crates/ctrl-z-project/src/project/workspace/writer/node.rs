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

use serde_json::{Map, Value};

use crate::project::manifest::node::Node;
use crate::project::Result;

use super::{Writable, Writer};

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Writable for Writer<'_, Node> {
    fn write<S>(&self, input: S) -> Result<String>
    where
        S: AsRef<str>,
    {
        let content = input.as_ref();
        let mut doc = serde_json::from_str::<Value>(content)?;
        // update_workspace_dependencies(&mut doc, versions);
        // NPM_update_package_version(&mut doc, versions);

        // Update package version if name matches
        if let Some(obj) = doc.as_object_mut() {
            if let Some(name) = obj.get("name").and_then(|n| n.as_str()) {
                if let Some(new_version) = self.items.get(name) {
                    obj.insert(
                        "version".to_string(),
                        Value::String(new_version.to_string()),
                    );
                }
            }

            // Update dependencies sections - only normal deps for now
            if let Some(deps) = obj
                .get_mut("dependencies")
                .and_then(|value| value.as_object_mut())
            {
                update_npm_dependency_table(deps, self);
            }
        }

        let mut content = serde_json::to_string_pretty(&doc)?;
        content.push('\n');

        // Try npm first
        let npm_result = std::process::Command::new("npm")
            .args(["install", "--package-lock-only", "--ignore-scripts"])
            .output();

        if let Ok(output) = npm_result {
            if output.status.success() {
                // @todo?
            }
        }

        Ok(content)
    }
}

// ----------------------------------------------------------------------------
// Functions
// ----------------------------------------------------------------------------

fn update_npm_dependency_table(
    map: &mut Map<String, Value>, writer: &Writer<Node>,
) {
    for (name, value) in map.iter_mut() {
        if let Some(version) = writer.items.get(name.as_str()) {
            *value = Value::String(format!("^{version}"));
        }
    }
}
