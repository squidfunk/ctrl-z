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

//! Summary builder.

use super::Summary;

// ----------------------------------------------------------------------------
// Structs
// ----------------------------------------------------------------------------

/// Summary builder.
#[derive(Debug)]
pub struct Builder {
    /// Body content.
    body: Option<String>,
}

// ----------------------------------------------------------------------------
// Implementations
// ----------------------------------------------------------------------------

impl Builder {
    /// Creates a summary builder.
    ///
    /// Note that the canonical way to create [`Summary`] is to invoke the
    /// [`Summary::builder`] method, which creates an instance of [`Builder`].
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::summary::Builder;
    ///
    /// // Create summary builder
    /// let mut builder = Builder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { body: None }
    }

    /// Sets the body of the summary.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::Summary;
    ///
    /// // Create summary builder
    /// let mut builder = Summary::builder();
    /// builder.body("Somebody should do something")?;
    /// ```
    pub fn body<B>(&mut self, body: B) -> &mut Self
    where
        B: AsRef<str>,
    {
        self.body = Some(body.as_ref().trim().to_string());
        self
    }

    /// Builds the summary.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::Summary;
    ///
    /// // Create summary builder
    /// let mut builder = Summary::builder();
    /// builder.body("Somebody should do something")?;
    ///
    /// // Create summary from builder
    /// let summary = builder.build()?;
    /// ```
    #[must_use]
    pub fn build(self) -> Summary {
        Summary { body: self.body }
    }
}

// ----------------------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------------------

impl Default for Builder {
    /// Creates a summary builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrl_z_changeset::summary::Builder;
    ///
    /// // Create summary builder
    /// let mut builder = Builder::default();
    /// ```
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
