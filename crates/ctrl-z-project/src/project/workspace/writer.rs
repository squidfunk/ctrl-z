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

//! Workspace writer.

use semver::Version;
use std::collections::BTreeMap;
use std::fs;
use std::marker::PhantomData;
use std::path::Path;

use crate::project::version::{Increment, VersionExt};
use crate::project::{Manifest, Project, Result};

use super::Workspace;

mod cargo;
mod node;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

pub trait Writable {
    fn write<S>(&self, input: S) -> Result<String>
    where
        S: AsRef<str>;
}

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

pub struct Writer<'a, T> {
    items: BTreeMap<&'a str, Version>,
    marker: PhantomData<T>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl<T> Workspace<T>
where
    T: Manifest,
{
    pub fn apply(&self, increments: &[Option<Increment>]) -> Writer<'_, T> {
        let mut items = BTreeMap::new();
        for (node, (_, project)) in self.projects.iter().enumerate() {
            if let Some(increment) = &increments[node] {
                let name = project.name().expect("invariant");
                let version =
                    project.version().expect("invariant").bump(*increment);
                items.insert(name, version);
            }
        }
        Writer { items, marker: PhantomData }
    }
}

// ----------------------------------------------------------------------------

impl<T> Project<T>
where
    T: Manifest,
    for<'a> Writer<'a, T>: Writable, // @todo: meg super ugly...
{
    pub fn write<P>(self, writer: Writer<T>) -> Result
    where
        P: AsRef<Path>,
    {
        let content = fs::read_to_string(&self.path)?;
        let updated = writer.write(content.as_str())?;
        fs::write(&self.path, updated)?;
        Ok(())
    }
}
