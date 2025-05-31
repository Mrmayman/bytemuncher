use crate::{Endianness, Muncher};
use std::io::{Error, Read};

macro_rules! impl_read_float {
    ($name_le:ident, $name_be:ident, $name_ne:ident, $name_endian:ident, $type:ty, $int:ty, $size:expr) => {
        #[doc = "Reads a `"]
        #[doc = stringify!($type)]
        #[doc = "` in Little Endian format\n\nFor more info on endianness see [`crate::Endianness`]."]
        pub fn $name_le(&mut self) -> Result<$type, Error> {
            let mut buf = [0u8; $size];
            self.reader.read_exact(&mut buf)?;
            Ok(<$type>::from_bits(<$int>::from_le_bytes(buf)))
        }

        #[doc = "Reads a `"]
        #[doc = stringify!($type)]
        #[doc = "` in Big Endian format\n\nFor more info on endianness see [`crate::Endianness`]."]
        pub fn $name_be(&mut self) -> Result<$type, Error> {
            let mut buf = [0u8; $size];
            self.reader.read_exact(&mut buf)?;
            Ok(<$type>::from_bits(<$int>::from_be_bytes(buf)))
        }

        #[doc = "Reads a `"]
        #[doc = stringify!($type)]
        #[doc = "` in Native Endian format (according to target platform)\n\nFor more info on endianness see [`crate::Endianness`]."]
        pub fn $name_ne(&mut self) -> Result<$type, Error> {
            let mut buf = [0u8; $size];
            self.reader.read_exact(&mut buf)?;
            Ok(<$type>::from_bits(<$int>::from_ne_bytes(buf)))
        }

        #[doc = "Reads a `"]
        #[doc = stringify!($type)]
        #[doc = "` with the endianness specified in `end`\n\nFor more info on endianness see [`crate::Endianness`]."]
        pub fn $name_endian(&mut self, end: Endianness) -> Result<$type, Error> {
            let mut buf = [0u8; $size];
            self.reader.read_exact(&mut buf)?;
            let bits = if end.is_le() {
                <$int>::from_le_bytes(buf)
            } else {
                <$int>::from_be_bytes(buf)
            };
            Ok(<$type>::from_bits(bits))
        }
    };
}

/// **Floating point values** (`f32`/`f64`)
impl<T: Read> Muncher<T> {
    impl_read_float!(
        read_f32_le,
        read_f32_be,
        read_f32_ne,
        read_f32_endian,
        f32,
        u32,
        4
    );
    impl_read_float!(
        read_f64_le,
        read_f64_be,
        read_f64_ne,
        read_f64_endian,
        f64,
        u64,
        8
    );
}
