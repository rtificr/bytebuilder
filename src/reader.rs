use crate::traits::BytesTrait;

pub struct ByteReader<'a> {
    pub bytes: &'a [u8],
    pub pos: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ByteReader {
            bytes,
            pos: 0,
        }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        if self.remaining() < 1 {
            return None;
        }
        let value = self.bytes[self.pos];
        self.pos += 1;
        Some(value)
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        if self.remaining() < 2 {
            return None;
        }
        let bytes = &self.bytes[self.pos..self.pos + 2];
        self.pos += 2;
        Some(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        if self.remaining() < 4 {
            return None;
        }
        let value = u32::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;
        Some(value)
    }

    pub fn read_u64(&mut self) -> Option<u64> {
        if self.remaining() < 8 {
            return None;
        }
        let value = u64::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ]);
        self.pos += 8;
        Some(value)
    }

    pub fn read_u128(&mut self) -> Option<u128> {
        if self.remaining() < 16 {
            return None;
        }
        let value = u128::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
            self.bytes[self.pos + 8],
            self.bytes[self.pos + 9],
            self.bytes[self.pos + 10],
            self.bytes[self.pos + 11],
            self.bytes[self.pos + 12],
            self.bytes[self.pos + 13],
            self.bytes[self.pos + 14],
            self.bytes[self.pos + 15],
        ]);
        self.pos += 16;
        Some(value)
    }

    pub fn read_i8(&mut self) -> Option<i8> {
        if self.remaining() < 1 {
            return None;
        }
        let value = self.bytes[self.pos] as i8;
        self.pos += 1;
        Some(value)
    }

    pub fn read_i16(&mut self) -> Option<i16> {
        if self.remaining() < 2 {
            return None;
        }
        let value = i16::from_le_bytes([self.bytes[self.pos], self.bytes[self.pos + 1]]);
        self.pos += 2;
        Some(value)
    }

    pub fn read_i32(&mut self) -> Option<i32> {
        if self.remaining() < 4 {
            return None;
        }
        let value = i32::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;
        Some(value)
    }

    pub fn read_i64(&mut self) -> Option<i64> {
        if self.remaining() < 8 {
            return None;
        }
        let value = i64::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ]);
        self.pos += 8;
        Some(value)
    }

    pub fn read_i128(&mut self) -> Option<i128> {
        if self.remaining() < 16 {
            return None;
        }
        let value = i128::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
            self.bytes[self.pos + 8],
            self.bytes[self.pos + 9],
            self.bytes[self.pos + 10],
            self.bytes[self.pos + 11],
            self.bytes[self.pos + 12],
            self.bytes[self.pos + 13],
            self.bytes[self.pos + 14],
            self.bytes[self.pos + 15],
        ]);
        self.pos += 16;
        Some(value)
    }

    pub fn read_f32(&mut self) -> Option<f32> {
        if self.remaining() < 4 {
            return None;
        }
        let value = f32::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
        ]);
        self.pos += 4;
        Some(value)
    }

    pub fn read_f64(&mut self) -> Option<f64> {
        if self.remaining() < 8 {
            return None;
        }
        let value = f64::from_le_bytes([
            self.bytes[self.pos],
            self.bytes[self.pos + 1],
            self.bytes[self.pos + 2],
            self.bytes[self.pos + 3],
            self.bytes[self.pos + 4],
            self.bytes[self.pos + 5],
            self.bytes[self.pos + 6],
            self.bytes[self.pos + 7],
        ]);
        self.pos += 8;
        Some(value)
    }

    pub fn read_bool(&mut self) -> Option<bool> {
        if self.remaining() < 1 {
            return None;
        }
        let value = self.bytes[self.pos] != 0;
        self.pos += 1;
        Some(value)
    }

    pub fn read_str(&mut self, len: usize) -> Option<String> {
        if self.remaining() < len {
            return None;
        }
        let value = String::from_utf8_lossy(&self.bytes[self.pos..self.pos + len]).to_string();
        self.pos += len;
        Some(value)
    }

    pub fn read_bytes(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.remaining() < len {
            return None;
        }
        let value = self.bytes[self.pos..self.pos + len].to_vec();
        self.pos += len;
        Some(value)
    }

    pub fn read_len_prefixed_str(&mut self) -> Option<String> {
        let len = self.read_u64()? as usize;
        self.read_str(len)
    }

    pub fn read_len_prefixed_bytes(&mut self) -> Option<Vec<u8>> {
        let len = self.read_u64()? as usize;
        self.read_bytes(len)
    }

    pub fn read<T: BytesTrait>(&mut self) -> Option<T> {
        let remaining = &self.bytes[self.pos..];
        let result = T::from_bytes(remaining)?;

        let consumed = result.byte_size();
        self.pos += consumed;
        
        Some(result)
    }

    pub fn remaining(&self) -> usize {
        self.bytes.len() - self.pos
    }

    pub fn reset(&mut self) {
        self.pos = 0;
    }

    pub fn skip(&mut self, len: usize) -> bool {
        if self.remaining() < len {
            return false;
        }
        self.pos += len;
        true
    }
}