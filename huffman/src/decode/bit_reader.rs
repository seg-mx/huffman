use std::io::{self, Read};

pub struct BitReader<T: Read> {
    reader: T,
    buffer: u8,
    buffer_len: usize,
}

impl<T: Read> BitReader<T> {
    pub fn new(reader: T) -> Self {
        Self {
            reader,
            buffer: 0,
            buffer_len: 0,
        }
    }

    fn read_internal_byte(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    pub fn read_bit(&mut self) -> io::Result<u8> {
        if self.buffer_len > 0 {
            let bit = self.buffer >> (self.buffer_len - 1);
            self.buffer &= 2u8
                .overflowing_pow(self.buffer_len as u32 - 1)
                .0
                .overflowing_sub(1)
                .0;
            self.buffer_len -= 1;

            return Ok(bit);
        }

        let byte = self.read_internal_byte()?;
        let bit = byte >> 7;
        self.buffer = byte & 127; // Last seven bits
        self.buffer_len = 7;

        Ok(bit)
    }
}
