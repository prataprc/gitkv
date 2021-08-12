use std::{fs, path};

use crate::{err_at, Error, Result};

pub fn walk<P, S, F>(root: P, state: S, mut callb: F) -> Result<S>
where
    P: AsRef<path::Path>,
    F: FnMut(&mut S, &fs::DirEntry, usize, usize) -> Result<()>,
{
    let depth = 0;
    do_walk(root, state, &mut callb, depth)
}

fn do_walk<P, S, F>(root: P, mut state: S, callb: &mut F, depth: usize) -> Result<S>
where
    P: AsRef<path::Path>,
    F: FnMut(&mut S, &fs::DirEntry, usize, usize) -> Result<()>,
{
    let mut subdirs = vec![];

    for (breath, entry) in err_at!(IO, fs::read_dir(root))?.enumerate() {
        let entry = err_at!(IO, entry)?;
        callb(&mut state, &entry, depth, breath)?;
        if err_at!(IO, entry.file_type())?.is_dir() {
            subdirs.push(entry)
        }
    }

    for subdir in subdirs.into_iter() {
        state = do_walk(subdir.path(), state, callb, depth + 1)?;
    }

    Ok(state)
}
