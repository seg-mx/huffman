use std::{
    collections::HashMap,
    io::{self, Write},
};

use super::prefix_code::PrefixCode;

pub struct TableWriter<T: Write> {
    writer: T,
}

impl<T: Write> TableWriter<T> {
    pub fn new(writer: T) -> Self {
        Self { writer }
    }

    pub fn write(&mut self, prefix_codes: &HashMap<u8, PrefixCode>) -> io::Result<()> {
        for (&character, prefix_code) in prefix_codes {
            self.writer.write_all(&[character])?;
            self.writer
                .write_all(prefix_code.len().to_string().as_bytes())?;
            self.writer.write_all(&[b';'])?;
            self.writer.write_all(prefix_code.to_string().as_bytes())?;
        }

        Ok(())
    }
}
