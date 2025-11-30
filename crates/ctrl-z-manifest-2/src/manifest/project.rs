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

//! Project.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::error::Result;
use super::Manifest;

mod members;
mod paths;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Project.
#[derive(Debug)]
pub struct Project<M>
where
    M: Manifest,
{
    /// Manifests.
    items: BTreeMap<PathBuf, M>, // @todo do we really need this? this might be the outcome?
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

// make member patterns absolute (prepend with workspace dir)
// then make relative again!
// impls into iterator!!! this gives us freedom over how we use it (k, v)

// we might impl project as a trait?? or Members as a trait?
// or project is the leaf node? project.path + data

// we might not need the reader!
impl<M> Project<M>
where
    M: Manifest,
{
    pub fn read<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        // this is the base path
        let path = path.as_ref().canonicalize()?;
        let manifest = M::from_str(&fs::read_to_string(&path)?)?;

        println!("Read manifest from {}", path.display());
        println!("Data: {:#?}", manifest);

        // we might call this function recursively...! performance is not a
        // concern, so simplicity wins.

        // for each member - resolve them, and then load them as well...
        let root = path.parent().expect("invariant");
        let iter = manifest.members().iter();
        let patterns: Vec<PathBuf> =
            iter.map(|pattern| root.join(pattern)).collect(); // @todo remove collect

        println!("Patterns: {:#?}", patterns);

        //

        Ok(Self { items: BTreeMap::new() })
    }
}
// impl Reader for Cargo {
//     /// Reads a Cargo manifest from the given path.
//     ///
//     /// @todo
//     fn read<P>(path: P) -> Result<Self>
//     where
//         P: AsRef<Path>,
//     {
//         let path = path.as_ref();
//         let content = fs::read_to_string(path)?;
//         Cargo::from_str(&content)
//     }
// }
