use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("rocksdb error: {0}")]
    RocksDb(#[from] rocksdb::Error),
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("key not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, StoreError>;

/// Wrapper around RocksDB for esph context persistence.
pub struct EsphStore {
    db: rocksdb::DB,
}

impl EsphStore {
    /// Open an existing RocksDB at the given path.
    pub fn open(path: &Path) -> Result<Self> {
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(false);
        let db = rocksdb::DB::open(&opts, path)?;
        Ok(Self { db })
    }

    /// Initialize a new RocksDB at the given path.
    pub fn init_new(path: &Path) -> Result<Self> {
        let mut opts = rocksdb::Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        let db = rocksdb::DB::open(&opts, path)?;
        Ok(Self { db })
    }

    /// Get a raw value by key.
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key.as_bytes())?)
    }

    /// Put a raw key-value pair.
    pub fn put(&self, key: &str, value: &[u8]) -> Result<()> {
        self.db.put(key.as_bytes(), value)?;
        Ok(())
    }

    /// Delete a key.
    pub fn delete(&self, key: &str) -> Result<()> {
        self.db.delete(key.as_bytes())?;
        Ok(())
    }

    /// Get a JSON-serialized value.
    pub fn get_json<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
        match self.get(key)? {
            Some(bytes) => {
                let value = serde_json::from_slice(&bytes)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Put a JSON-serialized value.
    pub fn put_json<T: serde::Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let bytes = serde_json::to_vec(value)?;
        self.put(key, &bytes)
    }

    /// Flush WAL to disk.
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }
}
