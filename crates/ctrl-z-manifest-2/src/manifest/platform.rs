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

//! Manifest loader.

use std::path::Path;

use super::error::Result;

pub mod cargo;

// ----------------------------------------------------------------------------
// Traits
// ----------------------------------------------------------------------------

// // maybe we call this entire thing Loader! and then impl Loader for Cargo!
// // or we abstract loader as its the same for all and defined in from-str...

// /// Manifest loader.
// pub trait Loader: Sized {
//     /// Reads a manifest from the given path.
//     ///
//     /// # Errors
//     ///
//     /// This method should return errors encountered when reading a manifest.
//     fn load<P>(path: P) -> Result<Self>
//     where
//         P: AsRef<Path>;
// }
