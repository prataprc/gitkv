use std::{file, path};

use crate::{Error, Result};

pub struct KV {
    repo_path: path::PathBuf,
    db_path: Option<path::PathBuf>,
    config: toml::Value,
    repo: git2::Repository,
}

pub fn create<P>(repo_path: P, config: toml::Value) -> Result<KV>
where
    P: AsRef<path::Path>,
{
    use git2::{Repository, RepositoryInitMode, RepositoryInitOptions};

    let repo_path: path::PathBuf = repo_path.as_ref().into();
    let db_path = config_to_dbpath(&config);

    let (mode, description) = match config["init"].as_table() {
        None => (RepositoryInitMode::SHARED_UMASK, "".to_string()),
        Some(config) => {
            let mode = match config["permissions"].as_str() {
                Some("shared_umask") => RepositoryInitMode::SHARED_UMASK,
                Some("shared_group") => RepositoryInitMode::SHARED_GROUP,
                Some("shared_all") => RepositoryInitMode::SHARED_ALL,
                Some(perms) => err_at!(Invalid, msg: "Invalid permission {}", perms)?,
                None => RepositoryInitMode::SHARED_UMASK,
            };
            let description = config["description"].as_str().unwrap_or("").to_string();
            (mode, description)
        }
    };

    let mut options = RepositoryInitOptions::new();
    options
        .bare(false)
        .no_reinit(true)
        .no_dotgit_dir(false)
        .mkdir(true)
        .mkpath(true)
        .mode(mode)
        .description(&description);

    // initialize a new repository for key-value access.
    let repo = err_at!(Fatal, Repository::init_opts(repo_path.clone(), &options))?;

    Ok(KV {
        repo_path: repo_path.into(),
        db_path,
        config,
        repo,
    })
}

pub fn open<P>(repo_path: P, config: toml::Value) -> Result<KV>
where
    P: AsRef<path::Path>,
{
    use git2::{Repository, RepositoryOpenFlags};

    let repo_path: path::PathBuf = repo_path.as_ref().into();
    let db_path = config_to_dbpath(&config);

    let mut flags = RepositoryOpenFlags::empty();
    flags.set(RepositoryOpenFlags::NO_SEARCH, true);

    // initialize a new repository for key-value access.
    let repo = {
        let ceiling_dirs = Vec::<String>::default().into_iter();
        err_at!(
            Fatal,
            Repository::open_ext(repo_path.clone(), flags, ceiling_dirs)
        )?
    };

    Ok(KV {
        repo_path: repo_path.into(),
        db_path,
        config,
        repo,
    })
}

fn config_to_dbpath(config: &toml::Value) -> Option<path::PathBuf> {
    match config["db_path"].as_str() {
        Some("") | None => None,
        Some(s) => Some(path::PathBuf::from(s)),
    }
}
