// Copyright (c) 2025 Zensical and contributors
//
// SPDX-License-Identifier: MIT

use git2::{ObjectType, Oid, Reference as GitReference};

use crate::git::error::Result;

/// Kind of git reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceKind {
    /// refs/tags/<name>
    Tag(String),
    /// refs/heads/<name>
    Branch(String),
    /// refs/remotes/<remote>/<name>
    RemoteBranch { remote: String, name: String },
    /// Any other reference (e.g., notes, stash, custom refs)
    Other(String),
}

/// Thin wrapper around git2::Reference.
// #[derive(Debug)]
pub struct Reference<'a> {
    inner: git2::Reference<'a>,
}

impl<'a> Reference<'a> {
    pub fn new(inner: git2::Reference<'a>) -> Self {
        Self { inner }
    }

    /// Full ref name (e.g., "refs/tags/v1.0.0").
    pub fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    /// Short name (libgit2’s shorthand), when available.
    pub fn shorthand(&self) -> Option<&str> {
        self.inner.shorthand()
    }

    /// Classify the reference into Tag, Branch, RemoteBranch, or Other.
    pub fn kind(&self) -> ReferenceKind {
        let name = match self.inner.name() {
            Some(n) => n,
            None => return ReferenceKind::Other(String::new()),
        };

        if let Some(short) = name.strip_prefix("refs/tags/") {
            ReferenceKind::Tag(short.to_string())
        } else if let Some(short) = name.strip_prefix("refs/heads/") {
            ReferenceKind::Branch(short.to_string())
        } else if let Some(rem) = name.strip_prefix("refs/remotes/") {
            // refs/remotes/<remote>/<branch>
            match rem.split_once('/') {
                Some((remote, branch)) => ReferenceKind::RemoteBranch {
                    remote: remote.to_string(),
                    name: branch.to_string(),
                },
                None => ReferenceKind::Other(name.to_string()),
            }
        } else {
            ReferenceKind::Other(name.to_string())
        }
    }

    /// Peel to a commit OID if possible (handles annotated tags).
    pub fn peeled_commit_oid(&self) -> Option<Oid> {
        // return a commit...
        match self.inner.peel(ObjectType::Commit) {
            Ok(obj) => Some(obj.id()),
            Err(_) => None,
        }
    }

    /// Access underlying git2::Reference.
    pub fn inner(&self) -> &GitReference<'a> {
        &self.inner
    }
}
