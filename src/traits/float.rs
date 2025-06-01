use super::ReadEndian;

macro_rules! impl_float {
    ($type:ty, $int:ty) => {
        impl ReadEndian for $type {
            fn read_endian(
                reader: &mut impl std::io::Read,
                end: crate::End,
            ) -> Result<Self, std::io::Error> {
                let mut buf = [0u8; std::mem::size_of::<Self>()];
                reader.read_exact(&mut buf)?;
                let bits = if end.is_le() {
                    <$int>::from_le_bytes(buf)
                } else {
                    <$int>::from_be_bytes(buf)
                };
                Ok(Self::from_bits(bits))
            }

            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            fn into_usize(self) -> usize {
                self as usize
            }
        }
    };
}

impl_float!(f32, u32);
impl_float!(f64, u64);

// Not stable yet:
// impl_float!(f16, u16);
// impl_float!(f128, u128);
