use crate::error::{DatabaseError, Result};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct Wal {
    file: File,
}

impl Wal {
    pub fn open<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
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
        self.file.write_all(key)?;
        self.file.write_all(value)?;
        self.file.sync_data()?;

        Ok(pos)
    }

    pub fn read_all(&mut self) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
        self.file.seek(SeekFrom::Start(0))?;
        let mut entries = Vec::new();
        let mut buf = Vec::new();
        self.file.read_to_end(&mut buf)?;

        let mut cursor = 0;
        while cursor < buf.len() {
            if cursor + 12 > buf.len() {
                break;
            }

            let checksum = u32::from_le_bytes(buf[cursor..cursor + 4].try_into().unwrap());
            let key_len =
                u32::from_le_bytes(buf[cursor + 4..cursor + 8].try_into().unwrap()) as usize;
            let val_len =
                u32::from_le_bytes(buf[cursor + 8..cursor + 12].try_into().unwrap()) as usize;

            let total_len = 12 + key_len + val_len;
            if cursor + total_len > buf.len() {
                break;
            }

            let header = &buf[cursor + 4..cursor + 12];
            let key = &buf[cursor + 12..cursor + 12 + key_len];
            let value = &buf[cursor + 12 + key_len..cursor + total_len];

            let mut hasher = crc32fast::Hasher::new();
            hasher.update(header);
            hasher.update(key);
            hasher.update(value);
            if hasher.finalize() != checksum {
                return Err(DatabaseError::CorruptedData);
            }

            entries.push((key.to_vec(), value.to_vec()));
            cursor += total_len;
        }

        Ok(entries)
    }
}
