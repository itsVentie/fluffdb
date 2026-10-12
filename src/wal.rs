use std::fs::{File, OpenOptions};
use std::io::{self, Seek, SeekFrom, Write};
use std::path::Path;

pub struct Wal {
    file: File,
}

impl Wal {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self { file })
    }

    pub fn append(&mut self, key: &[u8], value: &[u8]) -> io::Result<u64> {
        let key_len = key.len() as u32;
        let val_len = value.len() as u32;

        let mut header = Vec::with_capacity(8);
        header.extend_from_slice(&key_len.to_le_bytes());
        header.extend_from_slice(&val_len.to_le_bytes());

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&header);
        hasher.update(key);
        hasher.update(value);
        let checksum = hasher.finalize();

        let pos = self.file.seek(SeekFrom::End(0))?;

        self.file.write_all(&checksum.to_le_bytes())?;
        self.file.write_all(&header)?;

        Ok(pos)
    }
}
