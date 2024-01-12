use std::{
    collections::HashMap,
    io::{self, BufRead, ErrorKind},
    num::ParseIntError,
};

use crate::prefix_code::PrefixCode;

#[derive(Debug)]
pub enum TableReadError {
    IOError(io::Error),
    LengthParseError(ParseIntError),
}

impl From<io::Error> for TableReadError {
    fn from(error: io::Error) -> Self {
        Self::IOError(error)
    }
}

impl From<ParseIntError> for TableReadError {
    fn from(error: ParseIntError) -> Self {
        Self::LengthParseError(error)
    }
}

pub struct TableReader<T: BufRead> {
    reader: T,
}

impl<T: BufRead> TableReader<T> {
    pub fn new(reader: T) -> Self {
        Self { reader }
    }

    fn read_byte(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_bits(&mut self, length: usize) -> io::Result<PrefixCode> {
        let bytes_len = (length - 1) / 8 + 1;
        let mut bytes = vec![0u8; bytes_len];

        self.reader.read_exact(&mut bytes)?;

        let mut bits = bytes[0] as u64;
        let mut bits_len = 8;

        for byte in bytes.into_iter().skip(1).map(|byte| byte as u64) {
            bits = (bits << 8) | byte;
            bits_len += 8;
        }

        bits >>= bits_len - length;

        Ok(PrefixCode::new(bits, length))
    }

    pub fn read(&mut self) -> Result<HashMap<u8, PrefixCode>, TableReadError> {
        let mut map = HashMap::new();

        loop {
            let character = match self.read_byte() {
                Ok(character) => character,
                Err(err) if err.kind() == ErrorKind::UnexpectedEof => break,
                err => err?,
            };

            let mut length = Vec::new();
            self.reader.read_until(b';', &mut length)?;

            let length = length[..length.len() - 1]
                .iter()
                .map(|&byte| (byte as char).to_string())
                .collect::<String>()
                .parse::<usize>()?;

            let code = self.read_bits(length)?;

            map.insert(character, code);
        }

        Ok(map)
    }
}
