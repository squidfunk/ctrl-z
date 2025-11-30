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
use std::fmt::Debug;
use std::str::FromStr;

use super::error::Error;

pub mod cargo;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

/// Manifest.
///
/// @todo document that this includes workspace and package manifests...,
/// which is why things might not have names or versions (depending on ecosystem)
/// - also explain why we use a mix here, since ecosystems are different, so
/// tjhere's no clean cut between workspaces and members (e.g. in npm)
pub trait Manifest: Debug + FromStr<Err = Error> {
    /// Returns the name.
    fn name(&self) -> Option<&str>;
    /// Returns the version.
    fn version(&self) -> Option<&Version>;
    /// Returns the members.
    fn members(&self) -> &[String];
}
