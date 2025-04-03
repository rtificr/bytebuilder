use builder::ByteBuilder;
use reader::ByteReader;
use traits::BytesTrait;

mod builder;
mod reader;
mod traits;

#[cfg(test)]
mod tests {
    use crate::builder::ByteBuilder;
    use crate::reader::ByteReader;

    use super::*;

    #[test]
    fn it_works() {
        let mut bytes_in = ByteBuilder::new();
        bytes_in.push_u8(1);
        bytes_in.push_u16(2);
        bytes_in.push_u32(3);
        bytes_in.push_u64(4);
        bytes_in.push_u128(5);
        bytes_in.push_i8(-1);
        bytes_in.push_i16(-2);
        bytes_in.push_i32(-3);
        bytes_in.push_i64(-4);
        bytes_in.push_i128(-5);
        bytes_in.push_f32(1.0);
        bytes_in.push_f64(2.0);
        bytes_in.push_bool(true);
        bytes_in.push_str("Hello World!");
        bytes_in.push(S {
            str: String::new(),
            val: 1,
        });

        let bytes = &bytes_in.bytes;
        let mut bytes_out = ByteReader::new(bytes);
        assert_eq!(bytes_out.read_u8(), Some(1));
        assert_eq!(bytes_out.read_u16(), Some(2));
        assert_eq!(bytes_out.read_u32(), Some(3));
        assert_eq!(bytes_out.read_u64(), Some(4));
        assert_eq!(bytes_out.read_u128(), Some(5));
        assert_eq!(bytes_out.read_i8(), Some(-1));
        assert_eq!(bytes_out.read_i16(), Some(-2));
        assert_eq!(bytes_out.read_i32(), Some(-3));
        assert_eq!(bytes_out.read_i64(), Some(-4));
        assert_eq!(bytes_out.read_i128(), Some(-5));
        assert_eq!(bytes_out.read_f32(), Some(1.0));
        assert_eq!(bytes_out.read_f64(), Some(2.0));
        assert_eq!(bytes_out.read_bool(), Some(true));
        assert_eq!(bytes_out.read_str(12), Some("Hello World!".to_string()));
        assert_eq!(
            bytes_out.read::<S>(),
            Some(S {
                str: String::new(),
                val: 1
            })
        );
        assert_eq!(bytes_out.remaining(), 0);
        assert_eq!(bytes_out.read_bytes(0), Some(vec![]));
        assert_eq!(bytes_out.read_u8(), None); // Should return None when no more bytes
    }

    #[test]
    fn test_len_prefixed() {
        let mut builder = ByteBuilder::new();
        builder.push_len_prefixed_str("Hello");
        builder.push_len_prefixed_bytes(&[1, 2, 3]);

        let bytes = &builder.bytes;
        let mut reader = ByteReader::new(bytes);
        assert_eq!(reader.read_len_prefixed_str(), Some("Hello".to_string()));
        assert_eq!(reader.read_len_prefixed_bytes(), Some(vec![1, 2, 3]));
    }
}

#[derive(Debug, PartialEq)]
pub struct S {
    pub str: String,
    pub val: i32,
}

impl BytesTrait for S {
    fn to_bytes(&self) -> Vec<u8> {
        let mut b = ByteBuilder::new();
        b.push_len_prefixed_str(&self.str);
        b.push_i32(self.val);
        b.bytes
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let mut reader = ByteReader::new(bytes);
        Some(Self {
            str: reader.read_len_prefixed_str()?,
            val: reader.read_i32()?,
        })
    }
}