use crate::memtable::MemTable;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

pub struct SSTable {
    file: File,
    index: Vec<(Vec<u8>, u64)>,
}

impl SSTable {
    pub fn create<P: AsRef<Path>>(path: P, memtable: &MemTable) -> io::Result<Self> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;

        let mut index = Vec::new();

        for (key, value) in memtable.iter() {
            let offset = file.seek(SeekFrom::End(0))?;
            index.push((key.clone(), offset));

            let k_len = key.len() as u32;
            let v_len = value.len() as u32;

            file.write_all(&k_len.to_le_bytes())?;
            file.write_all(&v_len.to_le_bytes())?;
            file.write_all(key)?;
            file.write_all(value)?;
        }

        file.sync_all()?;

        Ok(Self { file, index })
    }

    pub fn get(&mut self, key: &[u8]) -> io::Result<Option<Vec<u8>>> {
        let idx = self.index.binary_search_by(|(k, _)| k.as_slice().cmp(key));

        let (_, offset) = match idx {
            Ok(i) => &self.index[i],
            Err(_) => return Ok(None),
        };

        self.file.seek(SeekFrom::Start(*offset))?;

        let mut len_buf = [0u8; 8];
        self.file.read_exact(&mut len_buf)?;

        let k_len = u32::from_le_bytes(len_buf[0..4].try_into().unwrap()) as usize;
        let v_len = u32::from_le_bytes(len_buf[4..8].try_into().unwrap()) as usize;

        let mut k_buf = vec![0u8; k_len];
        let mut v_buf = vec![0u8; v_len];

        self.file.read_exact(&mut k_buf)?;
        self.file.read_exact(&mut v_buf)?;

        if k_buf == key {
            Ok(Some(v_buf))
        } else {
            Ok(None)
        }
    }
}
