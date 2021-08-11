use std::{file, path};

pub struct OpenOptions {
    pub repo_path: path::Path,
    pub db_path: path::Path,
}

pub struct GitKV<K, V> {
    repo_path: path::Path,
    db_path: path::Path,
}

impl<K, V> GitKV<K, V> {
    fn open<P: AsRef<path::Path>(repo_path: P) -> Self {
        GitKV {
            repo_path: repo_path.as_ref().clone(),
            db_path: repo_path.as_ref().clone(),
        }
    }

    fn open_with_options(options: OpenOptions) -> Self {
        GitKV {
            repo_path: options.repo_path,
            db_path: options.db_path,
        }
    }
}

impl<K, V> GitKV<K, V> {
    pub fn path(&self) -> path::Path {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }

    pub fn len(&self) -> usize {
        todo!()
    }
}

impl<K, V> GitKV<K, V> {
    pub fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        todo!()
    }

    pub fn set(&mut self, key: K, value: V) -> Option<V>
    where
        K: PartialEq,
    {
        todo!()
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq,
    {
        todo!()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        todo!()
    }

    pub fn iter_keys(&self) -> IterKeys<'_, K> {
        todo!()
    }

    pub fn range<Q: ?Sized, R>(&self, range: R) -> Range<'_, K, V, R, Q>
    where
        K: Borrow<Q>,
        R: RangeBounds<Q>,
        Q: Ord,
    {
        todo!()
    }

    pub fn reverse<Q: ?Sized, R>(&self, range: R) -> Reverse<'_, K, V, R, Q>
    where
        K: Borrow<Q>,
        R: RangeBounds<Q>,
        Q: Ord,
    {
        todo!()
    }
}
