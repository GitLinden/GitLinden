use std::{path::Path, sync::Mutex};

use git2::{BranchType, Error};

pub struct Repository {
    repository: Mutex<git2::Repository>,
}

impl Repository {
    pub(crate) fn open(path: &Path) -> Result<Repository, Error> {
        let repository = git2::Repository::open(path)?;

        Ok(Self {
            repository: Mutex::new(repository),
        })
    }

    pub fn branches(&self) -> Vec<String> {
        let Ok(repository) = self.repository.try_lock() else {
            return Vec::new();
        };

        let Ok(branches) = repository.branches(Some(BranchType::Local)) else {
            return Vec::new();
        };

        branches
            .flat_map(|result| result.ok())
            .flat_map(|(branch, _)| branch.name().ok().flatten().map(|name| name.to_string()))
            .collect()
    }
}
