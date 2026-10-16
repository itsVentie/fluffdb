use crate::error::Result;
use crate::memtable::MemTable;
use crate::wal::Wal;
use std::path::Path;

pub struct Engine {
    wal: Wal,
    memtable: MemTable,
}

impl Engine {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut wal = Wal::open(&path)?;
        let entries = wal.read_all()?;

        let mut memtable = MemTable::new();
        for (key, value) in entries {
            memtable.insert(key, value);
        }

        Ok(Self { wal, memtable })
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<()> {
        self.wal.append(key, value)?;
        self.memtable.insert(key.to_vec(), value.to_vec());
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Option<Vec<u8>> {
        self.memtable.get(key).map(|v| v.to_vec())
    }
}
