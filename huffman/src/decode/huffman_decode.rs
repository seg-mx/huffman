use std::{
    collections::HashMap,
    io::{BufRead, Write},
};

use crate::prefix_code::PrefixCode;

use super::{
    data_reader::{DataReadError, DataReader},
    table_reader::{TableReadError, TableReader},
};

#[derive(Default)]
pub struct HuffmanDecode {
    prefix_codes: HashMap<PrefixCode, u8>,
}

impl HuffmanDecode {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read_table(&mut self, reader: &mut impl BufRead) -> Result<(), TableReadError> {
        let mut table_reader = TableReader::new(reader);
        self.prefix_codes = table_reader.read()?;
        Ok(())
    }

    pub fn decode_data(
        &mut self,
        data: &mut impl BufRead,
        output: &mut impl Write,
    ) -> Result<(), DataReadError> {
        let mut data_reader = DataReader::new(data);
        data_reader.read(output, &self.prefix_codes)
    }
}
