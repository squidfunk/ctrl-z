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

//! Cargo manifest writer.

use std::fs;

use semver::Version;
use toml_edit::DocumentMut;

use crate::manifest::format::Cargo;
use crate::manifest::Manifest;

use super::{Result, Writer};

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

// Manifest: to_writer <- make this a writer struct?

// @todo abstract that again?
impl Writer for Manifest<Cargo> {
    /// Updates the manifest's version.
    fn set_version(&self, version: Version) -> Result {
        let mut document: DocumentMut =
            fs::read_to_string(&self.path)?.parse()?;

        // what happens in case of workspace?
        let option = document.get_mut("package"); // we need to distinguish package and the other thing
        if let Some(package) = option.and_then(|x| x.as_table_like_mut()) {
            package.insert("version", toml_edit::value(version.to_string()));
            fs::write(&self.path, document.to_string())?;
            // return Ok(());
        }

        Ok(())
    }

    fn set_version_req(&self, name: &str, version: Version) -> Result {
        let mut document: DocumentMut =
            fs::read_to_string(&self.path)?.parse()?;

        // what happens in case of workspace?
        let option = document
            .get_mut("workspace")
            .and_then(|x| x.get_mut("dependencies"))
            .and_then(|x| x.get_mut(name));

        if let Some(dependency) = option.and_then(|x| x.as_table_like_mut()) {
            dependency.insert("version", toml_edit::value(version.to_string()));
            fs::write(&self.path, document.to_string())?;
        }
        Ok(())
    }
}
