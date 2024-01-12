use std::{
    collections::HashMap,
    io::{self, Write},
};

use crate::prefix_code::PrefixCode;

pub struct DataWriter<T: Write> {
    writer: T,
    buffer: u8,
    buffer_len: usize,
}

impl<T: Write> DataWriter<T> {
    pub fn new(writer: T) -> Self {
        Self {
            writer,
            buffer: 0,
            buffer_len: 0,
        }
    }

    pub fn write(&mut self, prefix_codes: &HashMap<u8, PrefixCode>, data: &[u8]) -> io::Result<()> {
        let len = data
            .iter()
            .map(|byte| prefix_codes.get(byte).unwrap())
            .fold(0, |acc, code| acc + code.len());

        write!(self.writer, "{len};")?;

        for byte in data {
            self.write_prefix_code(prefix_codes.get(byte).unwrap())?;
        }

        self.flush()?;

        Ok(())
    }

    fn write_prefix_code(&mut self, code: &PrefixCode) -> io::Result<()> {
        if code.len() == 0 {
            return Ok(());
        }

        if self.buffer_len + code.len() < 8 {
            let bits = code.bits() as u8;
            self.buffer = (self.buffer << code.len()) | bits;
            self.buffer_len += code.len();
            return Ok(());
        }

        let mut bits = code.bits();
        let mut code_len = code.len();

        if self.buffer_len > 0 {
            let shift = code_len - (8 - self.buffer_len);
            self.writer
                .write_all(&[(self.buffer << (8 - self.buffer_len)) | (bits >> shift) as u8])?;

            bits &= 2u64.overflowing_pow(shift as u32).0.overflowing_sub(1).0;
            code_len = shift;
            self.buffer = 0;
            self.buffer_len = 0;
        }

        while code_len >= 8 {
            self.writer.write_all(&[(bits >> (code_len - 8)) as u8])?;
            code_len -= 8;
            bits &= 2u64.pow(code_len as u32) - 1;
        }

        self.buffer = bits as u8;
        self.buffer_len = code_len;

        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.buffer_len == 0 {
            return Ok(());
        }

        self.writer
            .write_all(&[self.buffer << (8 - self.buffer_len)])?;
        self.buffer = 0;
        self.buffer_len = 0;

        Ok(())
    }
}
