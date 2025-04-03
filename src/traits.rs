pub trait BytesTrait: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
    fn byte_size(&self) -> usize {
        self.to_bytes().len()
    }
}