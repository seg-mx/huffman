#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrefixCode {
    bits: u64,
    len: usize,
}

impl PrefixCode {
    pub fn new(bits: u64, len: usize) -> Self {
        Self { bits, len }
    }

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

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let shift = (self.len - 1) / 8 * 8 + 8 - self.len;
        let bits = self.bits << shift;
        let mut len = self.len + shift;

        while len != 0 {
            bytes.push((bits >> (len - 8)) as u8);
            len -= 8;
        }

        bytes
    }
}
