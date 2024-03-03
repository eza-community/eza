use hgrs::{FileStatus, MercurialRepository};
use std::path::PathBuf;

use crate::fs::fields as f;

#[derive(Debug, Clone)]
pub struct MercurialCache {
    pub repos: Vec<MercurialRepo>,

    pub misses: Vec<PathBuf>,
}

impl MercurialCache {
    pub fn get(&self, file_path: &PathBuf) -> f::Mercurial {
        f::Mercurial {
            status: self
                .repos
                .iter()
                .find(|r| r.has_path(file_path))
                .map(|r| r.get(file_path))
                .unwrap_or_default()
                .into(),
        }
    }
}

impl FromIterator<PathBuf> for MercurialCache {
    fn from_iter<T: IntoIterator<Item = PathBuf>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut mercurial = Self {
            repos: Vec::with_capacity(iter.size_hint().0),
            misses: Vec::new(),
        };

        for path in iter {
            match MercurialRepo::discover(&path) {
                Ok(repo) => mercurial.repos.push(repo),
                Err(path) => mercurial.misses.push(path),
            }
        }
        mercurial
    }
}

#[derive(Debug, Clone)]
pub struct MercurialRepo {
    pub repo: MercurialRepository,
    pub path: PathBuf,
    pub extra_paths: Vec<PathBuf>,
}

impl MercurialRepo {
    pub fn get(&self, file_path: &PathBuf) -> FileStatus {
        self.repo.get_status(file_path)
    }

    pub fn has_path(&self, file_path: &PathBuf) -> bool {
        let dir = file_path.parent().unwrap();

        if dir == self.path {
            return true;
        }
        if self.extra_paths.contains(&dir.into()) {
            return true;
        }
        false
    }

    pub fn discover(path: &PathBuf) -> Result<Self, PathBuf> {
        let r = match hgrs::find_repo_recursively(path, 10) {
            Some(r) => r,
            None => return Err(path.clone()),
        };
        Ok(Self {
            repo: r,
            path: path.into(),
            extra_paths: Vec::new(),
        })
    }
}
