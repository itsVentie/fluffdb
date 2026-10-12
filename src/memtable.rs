use std::collections::BTreeMap;


#[derive(Default)]
pub struct MemTable {
    map: BTreeMap<Vec<u8>, Vec<u8>>,
    byte_size: usize,
}

impl MemTable {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            byte_size: 0,
        }
    }

    pub fn insert(&mut self, key: Vec<u8>, value: Vec<u8>) {
        let k_len = key.len();
        let v_len = value.len();
        
        if let Some(old_val) = self.map.insert(key, value) {
            self.byte_size = self.byte_size - old_val.len() + v_len;
        } else {
            self.byte_size += k_len + v_len;
        }
    }

    pub fn get(&self, key: &[u8]) -> Option<&[u8]> {
        self.map.get(key).map(|v| v.as_slice())
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn byte_size(&self) -> usize {
        self.byte_size
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Vec<u8>, &Vec<u8>)> {
        self.map.iter()
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.byte_size = 0;
    }
}