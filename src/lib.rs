pub mod asset_bundle;
pub mod unity_cn;

pub struct BinaryReader<'a> {
    buffer: &'a [u8],
    cursor: usize,
}

impl<'a> BinaryReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self { buffer, cursor: 0 }
    }

    pub fn read_bytes(&mut self, len: usize) -> &'a [u8] {
        let cursor = self.cursor;
        self.cursor += len;

        &self.buffer[cursor..self.cursor]
    }

    pub fn read_u16(&mut self) -> u16 {
        let bits = u16::BITS as usize;

        self.read_bytes(bits / 8)
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as u16) << (bits - (i + 1) * 8))
            .sum()
    }

    pub fn read_u32(&mut self) -> u32 {
        let bits = u32::BITS as usize;

        self.read_bytes(bits / 8)
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as u32) << (bits - (i + 1) * 8))
            .sum()
    }

    pub fn read_u64(&mut self) -> u64 {
        let bits = u64::BITS as usize;

        self.read_bytes(bits / 8)
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as u64) << (bits - (i + 1) * 8))
            .sum()
    }

    pub fn read_i16(&mut self) -> i16 {
        let bits = i16::BITS as usize;

        self.read_bytes(bits / 8)
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as i16) << (bits - (i + 1) * 8))
            .sum()
    }

    pub fn read_i32(&mut self) -> i32 {
        let bits = i32::BITS as usize;

        self.read_bytes(bits / 8)
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as i32) << (bits - (i + 1) * 8))
            .sum()
    }

    pub fn read_i64(&mut self) -> i64 {
        let bits = i64::BITS as usize;

        self.read_bytes(bits / 8)
            .iter()
            .enumerate()
            .map(|(i, &byte)| (byte as i64) << (bits - (i + 1) * 8))
            .sum()
    }

    pub fn read_cstr(&mut self) -> String {
        let cursor = self.cursor;

        while self.buffer[self.cursor] != 0 {
            self.cursor += 1;
        }

        let result = self.buffer[cursor..self.cursor].to_vec();
        self.cursor += 1;

        unsafe { String::from_utf8_unchecked(result) }
    }
}

#[cfg(test)]
mod tests;
