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

    pub fn read_string(&mut self, len: usize) -> String {
        let cursor = self.cursor;
        self.cursor += len;

        String::from_iter(
            self.buffer[cursor..self.cursor]
                .iter()
                .map(|&byte| byte as char),
        )
    }

    pub fn read_string_to_null(&mut self) -> String {
        let mut result = String::new();

        while {
            let byte = self.buffer[self.cursor];
            self.cursor += 1;

            0.ne(&byte).then(|| result.push(byte as char)).is_some()
        } {}

        result
    }
}
