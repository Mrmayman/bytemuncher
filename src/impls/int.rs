use std::io::{Error, ErrorKind, Read};

use crate::{Endianness, Muncher};

macro_rules! impl_read_int {
    ($fn_name:ident, $type:ty, $size:expr, $from_bytes:ident, $en:expr) => {
        #[doc = "Reads `"]
        #[doc = stringify!($type)]
        #[doc = "` integer as "]
        #[doc = $en]
        #[doc = "\n\nFor more info on endianness see [`crate::Endianness`]."]
        pub fn $fn_name(&mut self) -> Result<$type, Error> {
            let mut buf = [0u8; $size];
            self.reader.read_exact(&mut buf)?;
            Ok(<$type>::$from_bytes(buf))
        }
    };
}

macro_rules! impl_read_int_endian {
    ($name:ident, $type:ty, $size:expr) => {
        #[doc = "Reads `"]
        #[doc = stringify!($type)]
        #[doc = "` integer with `end` endianness.\n\nFor more info on endianness see [`crate::Endianness`]."]

        pub fn $name(&mut self, end: Endianness) -> Result<$type, Error> {
            let mut buf = [0u8; $size];
            self.reader.read_exact(&mut buf)?;
            Ok(if end.is_le() {
                <$type>::from_le_bytes(buf)
            } else {
                <$type>::from_be_bytes(buf)
            })
        }
    };
}

/// **Integer and boolean read methods**
impl<T: Read> Muncher<T> {
    /// Reads a `u8` (unsigned byte)
    pub fn read_u8(&mut self) -> Result<u8, Error> {
        let mut buf = [0];
        self.reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    /// Reads an `i8` (signed byte)
    pub fn read_i8(&mut self) -> Result<i8, Error> {
        Ok(self.read_u8()? as i8)
    }

    impl_read_int!(read_u16_le, u16, 2, from_le_bytes, "Little Endian");
    impl_read_int!(read_u16_be, u16, 2, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_u16_ne,
        u16,
        2,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );
    impl_read_int!(read_i16_le, i16, 2, from_le_bytes, "Little Endian");
    impl_read_int!(read_i16_be, i16, 2, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_i16_ne,
        i16,
        2,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );

    impl_read_int!(read_u32_le, u32, 4, from_le_bytes, "Little Endian");
    impl_read_int!(read_u32_be, u32, 4, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_u32_ne,
        u32,
        4,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );
    impl_read_int!(read_i32_le, i32, 4, from_le_bytes, "Little Endian");
    impl_read_int!(read_i32_be, i32, 4, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_i32_ne,
        i32,
        4,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );

    impl_read_int!(read_u64_le, u64, 8, from_le_bytes, "Little Endian");
    impl_read_int!(read_u64_be, u64, 8, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_u64_ne,
        u64,
        8,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );
    impl_read_int!(read_i64_le, i64, 8, from_le_bytes, "Little Endian");
    impl_read_int!(read_i64_be, i64, 8, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_i64_ne,
        i64,
        8,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );

    impl_read_int!(read_u128_le, u128, 16, from_le_bytes, "Little Endian");
    impl_read_int!(read_u128_be, u128, 16, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_u128_ne,
        u128,
        16,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );
    impl_read_int!(read_i128_le, i128, 16, from_le_bytes, "Little Endian");
    impl_read_int!(read_i128_be, i128, 16, from_be_bytes, "Big Endian");
    impl_read_int!(
        read_i128_ne,
        i128,
        16,
        from_ne_bytes,
        "Native Endian (according to target platform)"
    );

    impl_read_int_endian!(read_u16_endian, u16, 2);
    impl_read_int_endian!(read_i16_endian, i16, 2);
    impl_read_int_endian!(read_u32_endian, u32, 4);
    impl_read_int_endian!(read_i32_endian, i32, 4);
    impl_read_int_endian!(read_u64_endian, u64, 8);
    impl_read_int_endian!(read_i64_endian, i64, 8);
    impl_read_int_endian!(read_u128_endian, u128, 16);
    impl_read_int_endian!(read_i128_endian, i128, 16);

    /// Reads a `bool` (`true`/`false`) value.
    ///
    /// This reads a byte.
    /// - If the byte is `0x00` then `false`
    /// - If the byte is `0x01` then `true`
    ///
    /// If you want different behaviour
    /// (eg: any non zero value being true)
    /// then use [`Muncher::read_u8`]
    /// and manually do the check.
    pub fn read_bool(&mut self) -> Result<bool, Error> {
        let byte = self.read_u8()?;
        match byte {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid bool byte: {:#X}", byte),
            )),
        }
    }
}
