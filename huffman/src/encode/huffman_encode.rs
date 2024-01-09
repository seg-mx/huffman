use std::{
    collections::HashMap,
    fmt,
    io::{self, Write},
};

use super::{table_writer::TableWriter, tree::Tree};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HuffmanEncodeCreationError {
    EmptyBytes,
}

impl fmt::Display for HuffmanEncodeCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for HuffmanEncodeCreationError {}

pub struct HuffmanEncode {
    frequencies: HashMap<u8, u64>,
    bytes: Vec<u8>,
    tree: Option<Tree>,
}

impl HuffmanEncode {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, HuffmanEncodeCreationError> {
        if bytes.is_empty() {
            return Err(HuffmanEncodeCreationError::EmptyBytes);
        }

        let mut frequencies = HashMap::new();

        for &byte in bytes {
            *frequencies.entry(byte).or_insert(0) += 1;
        }

        Ok(Self {
            frequencies,
            bytes: bytes.to_vec(),
            tree: None,
        })
    }

    pub fn write_table(&mut self, address: &mut impl Write) -> io::Result<()> {
        if self.tree.is_none() {
            self.tree = Some(Tree::from_frequencies(&self.frequencies).unwrap());
        }

        TableWriter::new(address).write(self.tree.as_mut().unwrap().get_prefix_codes())
    }

    pub fn write_data(&self, address: &mut impl Write) -> io::Result<()> {
        todo!()
    }
}
