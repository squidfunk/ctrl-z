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

//! Summary.

use std::fmt;

mod builder;

pub use builder::Builder;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Summary.
#[derive(Debug)]
pub struct Summary {
    /// Body content.
    body: Option<String>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Summary {
    /// Creates a summary builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::Summary;
    ///
    /// // Create summary builder
    /// let mut builder = Summary::builder();
    #[inline]
    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

#[allow(clippy::must_use_candidate)]
impl Summary {
    /// Returns the body content.
    #[inline]
    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl<S> From<S> for Summary
where
    S: AsRef<str>,
{
    /// Creates a summary from a string.
    #[inline]
    fn from(value: S) -> Self {
        let mut builder = Self::builder();
        builder.body(value);
        builder.build()
    }
}

// ----------------------------------------------------------------------------

impl fmt::Display for Summary {
    /// Formats the summary for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(body) = &self.body {
            body.trim().fmt(f)?;
        }

        // No errors occurred
        Ok(())
    }
}
