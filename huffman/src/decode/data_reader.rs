use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
    num::ParseIntError,
};

use crate::prefix_code::PrefixCode;

use super::bit_reader::BitReader;

#[derive(Debug)]
pub enum DataReadError {
    IOError(io::Error),
    LengthParseError(ParseIntError),
}

impl From<io::Error> for DataReadError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<ParseIntError> for DataReadError {
    fn from(error: ParseIntError) -> Self {
        Self::LengthParseError(error)
    }
}

pub struct DataReader<T: BufRead> {
    reader: T,
}

impl<T: BufRead> DataReader<T> {
    pub fn new(reader: T) -> Self {
        Self { reader }
    }

    pub fn read(
        &mut self,
        address: &mut impl Write,
        prefix_codes: &HashMap<PrefixCode, u8>,
    ) -> Result<(), DataReadError> {
        let mut amount = Vec::new();

        self.reader.read_until(b';', &mut amount)?;

        let amount = amount[..amount.len() - 1]
            .iter()
            .map(|&byte| (byte as char).to_string())
            .collect::<String>()
            .parse::<usize>()?;

        let mut bit_reader = BitReader::new(&mut self.reader);

        let mut code = PrefixCode::default();

        for _ in (1..=amount).rev() {
            let bit = bit_reader.read_bit()?;
            code = if bit == 1 {
                code.add_one()
            } else {
                code.add_zero()
            };

            if let Some(&character) = prefix_codes.get(&code) {
                address.write_all(&[character])?;
                code = PrefixCode::default();
            }
        }

        Ok(())
    }
}
