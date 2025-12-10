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

//! Manifest.

use semver::Version;
use std::borrow::Cow;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::error::Error;

pub mod cargo;
pub mod node;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

/// Manifest.
///
/// Manifests are packages and workspaces – sometimes one or the other, and
/// sometimes both at the same time, depending on the ecosystem. This is also
/// why several methods of this trait return optional references – ecosystems
/// differ in how they implement these concepts (e.g. Rust and Node).
///
/// Note that manifests only return the names of their dependencies, not their
/// version requirements, since we only require inner-workspace dependencies,
/// which we resolve as part of workspace resolution. Also, some ecosystems
/// like Rust support inheriting version requirements from the workspace.
///
/// Think of this trait as being an adapter into an ecosystem-specific manifest
/// format, providing just enough information for version management.
pub trait Manifest: Debug + FromStr<Err = Error> {
    /// Resolves the manifest path from the given path.
    ///
    /// Some ecosystems allow for multiple names of manifests. This trait keeps
    /// resolution flexible, so we can even validate paths before resolution.
    ///
    /// # Errors
    ///
    /// This method should return an error if the path cannot be resolved, or
    /// if the file doesn't exist. Mechanics are up to the implementor.
    fn resolve(path: &Path) -> Result<PathBuf, Error>;

    /// Returns a reference to the name.
    fn name(&self) -> Option<&str>;

    /// Returns a reference to the version.
    fn version(&self) -> Option<&Version>;

    /// Returns a reference to the members.
    fn members(&self) -> Cow<'_, [String]>;

    /// Creates an iterator over the dependencies.
    fn dependencies(&self) -> impl Iterator<Item = &str>;
}
