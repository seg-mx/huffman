use std::fmt;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefixCode {
    bits: u64,
    len: usize,
}

impl PrefixCode {
    pub fn bits(&self) -> u64 {
        self.bits
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn add_zero(&self) -> Self {
        Self {
            len: if self.len == 64 { 64 } else { self.len + 1 },
            bits: self.bits << 1,
        }
    }

    pub fn add_one(&self) -> Self {
        Self {
            len: if self.len == 64 { 64 } else { self.len + 1 },
            bits: self.bits << 1 | 1,
        }
    }
}

impl fmt::Display for PrefixCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let shift = (self.len - 1) / 8 * 8 + 8 - self.len;
        let bits = self.bits << shift;
        let mut len = self.len + shift;

        while len != 0 {
            write!(f, "{}", (bits >> (len - 8)) as u8 as char)?;
            len -= 8;
        }

        Ok(())
    }
}
