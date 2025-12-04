// Copyright (c) 2025 Zensical and contributors
//
// SPDX-License-Identifier: MIT

use git2::{ObjectType, Oid, Reference as GitReference};

use crate::repository::commit::Commit;
use crate::repository::Result;

mod iter;

/// Kind of git reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceKind {
    /// refs/tags/<name>
    Tag { name: String },
    /// refs/heads/<name>
    Branch { name: String },
    /// refs/remotes/<remote>/<name>
    Remote { remote: String, name: String },
    /// Any other reference (e.g., notes, stash, custom refs)
    Other { name: String },
}

/// Thin wrapper around git2::Reference.
// #[derive(Debug)]
pub struct Reference<'a> {
    repository: &'a git2::Repository,
    inner: git2::Reference<'a>,
}

// implement get_reference and get_commit on Repo? then impl this instead of ::new.

impl<'a> Reference<'a> {
    // @todo construct this with out ::new
    pub fn new(
        repository: &'a git2::Repository, inner: git2::Reference<'a>,
    ) -> Self {
        Self { repository, inner }
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
            None => return ReferenceKind::Other { name: String::new() },
        };

        if let Some(short) = name.strip_prefix("refs/tags/") {
            ReferenceKind::Tag { name: short.to_string() }
        } else if let Some(short) = name.strip_prefix("refs/heads/") {
            ReferenceKind::Branch { name: short.to_string() }
        } else if let Some(rem) = name.strip_prefix("refs/remotes/") {
            // refs/remotes/<remote>/<branch>
            match rem.split_once('/') {
                Some((remote, branch)) => ReferenceKind::Remote {
                    remote: remote.to_string(),
                    name: branch.to_string(),
                },
                None => ReferenceKind::Other { name: name.to_string() },
            }
        } else {
            ReferenceKind::Other { name: name.to_string() }
        }
    }

    pub fn is_tag(&self) -> bool {
        self.inner.is_tag()
    }

    pub fn is_branch(&self) -> bool {
        self.inner.is_branch()
    }

    pub fn is_remote(&self) -> bool {
        self.inner.is_remote()
    }

    /// Peel to a commit OID if possible (handles annotated tags).
    pub fn commit(&self) -> Option<Result<Commit<'_>>> {
        // @todo: make option an error...?
        // return a commit...
        match self.inner.peel(ObjectType::Commit) {
            Ok(obj) => Some(Commit::new(self.repository, obj.id())),
            Err(_) => None,
        }
    }

    /// Access underlying git2::Reference.
    pub fn inner(&self) -> &GitReference<'a> {
        &self.inner
    }
}
