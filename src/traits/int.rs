use super::ReadEndian;

macro_rules! impl_small_int {
    ($type:ty) => {
        impl ReadEndian for $type {
            // Ignoring endianness here as it doesn't matter
            fn read_endian(
                reader: &mut impl std::io::Read,
                _: crate::End,
            ) -> Result<Self, std::io::Error>
            where
                Self: Sized,
            {
                let mut buf = [0];
                reader.read_exact(&mut buf)?;
                #[allow(clippy::cast_possible_wrap)]
                Ok(buf[0] as Self)
            }

            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            fn into_usize(self) -> usize {
                self as usize
            }
        }
    };
}

macro_rules! impl_int {
    ($type:ty) => {
        impl ReadEndian for $type {
            fn read_endian(
                reader: &mut impl std::io::Read,
                end: crate::End,
            ) -> Result<Self, std::io::Error>
            where
                Self: Sized,
            {
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                Ok(if end.is_le() {
                    Self::from_le_bytes(buf)
                } else {
                    Self::from_be_bytes(buf)
                })
            }

            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            fn into_usize(self) -> usize {
                self as usize
            }
        }
    };
}

impl_small_int!(u8);
impl_small_int!(i8);

impl_int!(u16);
impl_int!(i16);

impl_int!(u32);
impl_int!(i32);

impl_int!(u64);
impl_int!(i64);

impl_int!(u128);
impl_int!(i128);

// Not implemented, because if you're parsing binary formats
// you better know the type size beforehand!

// impl_int!(usize);
// impl_int!(isize);
