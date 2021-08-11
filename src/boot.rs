use std::{file, path};

/// Type defines how to initialize a new git repository.
pub struct InitOptions {
    repo_path: path::Path,
    db_path: path::Path, // relative to repo_path
    options: git2::RepositoryInitOptions,
}

impl InitOptions {
    pub fn new<P: AsRef<path::Path>(repo_path: P) -> InitOptions {
        let mut options = git2::RepositoryInitOptions::new()
        options.no_reinit().mkdir().mkpath()
        InitOptions {
            repo_path: repo_path.as_ref().clone(),
            db_path: repo_path.as_ref().clone(),
            options,
        }
    }

    pub fn set_description(mut self, description: &str) -> Self {
        self.options.description(description)
        self
    }
}


impl InitOptions {
    pub fn db_path(&self) -> path::Path {
        self.repo_path.join(&self.db_path).into()
    }
}

impl InitOptions {
    pub fn build(self) -> Result<()> {
        todo!()
    }
}
