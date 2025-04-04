use crate::traits::Byteable;

pub struct ByteBuilder {
    pub bytes: Vec<u8>,
}

impl ByteBuilder {
    pub fn new() -> Self {
        ByteBuilder {
            bytes: Vec::new(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        ByteBuilder {
            bytes: Vec::with_capacity(capacity),
        }
    }

    pub fn push_u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn push_u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_u128(&mut self, value: u128) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i8(&mut self, value: i8) {
        self.bytes.push(value as u8);
    }

    pub fn push_i16(&mut self, value: i16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i32(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i64(&mut self, value: i64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_i128(&mut self, value: i128) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_f32(&mut self, value: f32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_f64(&mut self, value: f64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn push_bool(&mut self, value: bool) {
        self.bytes.push(value as u8);
    }

    pub fn push_str(&mut self, value: &str) {
        self.bytes.extend_from_slice(value.as_bytes());
    }

    pub fn push_bytes(&mut self, value: &[u8]) {
        self.bytes.extend_from_slice(value);
    }

    pub fn push<T: Byteable>(&mut self, value: T) {
        self.bytes.extend_from_slice(value.to_bytes().as_slice());
    }
    
    pub fn push_asref<T: AsRef<[u8]>>(&mut self, value: T) {
        self.bytes.extend_from_slice(value.as_ref());
    }

    pub fn push_len_prefixed_str(&mut self, value: &str) {
        self.push_u64(value.len() as u64);
        self.push_str(value);
    }

    pub fn push_len_prefixed_bytes(&mut self, value: &[u8]) {
        self.push_u64(value.len() as u64);
        self.push_bytes(value);
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }
    
    pub fn with_u8(&mut self, value: u8) -> &mut Self {
        self.push_u8(value);
        self
    }
    pub fn with_u16(&mut self, value: u16) -> &mut Self {
        self.push_u16(value);
        self
    }
    pub fn with_u32(&mut self, value: u32) -> &mut Self {
        self.push_u32(value);
        self
    }
    pub fn with_u64(&mut self, value: u64) -> &mut Self {
        self.push_u64(value);
        self
    }
    pub fn with_u128(&mut self, value: u128) -> &mut Self {
        self.push_u128(value);
        self
    }
    pub fn with_i8(&mut self, value: i8) -> &mut Self {
        self.push_i8(value);
        self
    }
    pub fn with_i16(&mut self, value: i16) -> &mut Self {
        self.push_i16(value);
        self
    }
    pub fn with_i32(&mut self, value: i32) -> &mut Self {
        self.push_i32(value);
        self
    }
    pub fn with_i64(&mut self, value: i64) -> &mut Self {
        self.push_i64(value);
        self
    }
    pub fn with_i128(&mut self, value: i128) -> &mut Self {
        self.push_i128(value);
        self
    }
    pub fn with_f32(&mut self, value: f32) -> &mut Self {
        self.push_f32(value);
        self
    }
    pub fn with_f64(&mut self, value: f64) -> &mut Self {
        self.push_f64(value);
        self
    }
    pub fn with_bool(&mut self, value: bool) -> &mut Self {
        self.push_bool(value);
        self
    }
    pub fn with_str(&mut self, value: &str) -> &mut Self {
        self.push_str(value);
        self
    }
    pub fn with_bytes(&mut self, value: &[u8]) -> &mut Self {
        self.push_bytes(value);
        self
    }
    pub fn with_len_prefixed_str(&mut self, value: &str) -> &mut Self {
        self.push_len_prefixed_str(value);
        self
    }
    pub fn with_len_prefixed_bytes(&mut self, value: &[u8]) -> &mut Self {
        self.push_len_prefixed_bytes(value);
        self
    }
    pub fn with<T: Byteable>(&mut self, value: T) -> &mut Self {
        self.push(value);
        self
    }
    pub fn with_asref<T: AsRef<[u8]>>(&mut self, value: T) -> &mut Self {
        self.push_asref(value);
        self
    }
}
impl From<&[u8]> for ByteBuilder {
    fn from(value: &[u8]) -> Self {
        ByteBuilder {
            bytes: value.to_vec(),
        }
    }
}
impl From<Vec<u8>> for ByteBuilder {
    fn from(value: Vec<u8>) -> Self {
        ByteBuilder { bytes: value }
    }
}