use std::{
    borrow::Cow,
    io::{BufRead, Read, Write},
};

use crate::{End, Muncher, Primitive};

mod error;

pub use error::MutfError;

#[cfg(feature = "futures")]
use futures::io::{AsyncBufRead, AsyncRead, AsyncWriteExt};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncBufRead, AsyncRead, AsyncWriteExt};

#[cfg(any(feature = "tokio", feature = "futures"))]
use crate::traits::AsyncPrimitive;

/// **Size-prefixed MUTF-8 string methods**
impl<T: Read> Muncher<T> {
    /// Reads a MUTF-8 string prefixed by a length (number of bytes) of type `<E>`,
    /// then converts it to a UTF-8 [`String`].
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want raw MUTF-8, use [`Muncher::read_pref_bytes`].
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub fn read_pref_mutf8<E: Primitive>(&mut self, end: End) -> Result<String, MutfError> {
        let buf = self.read_pref_bytes::<E>(end)?;
        mutf2utf(&buf)
    }

    /// Reads `len` number of bytes into a MUTF-8 string,
    /// then converts it to a UTF-8 [`String`].
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want raw MUTF-8, use [`Muncher::read_fixed_bytes`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub fn read_fixed_mutf8(&mut self, len: usize) -> Result<String, MutfError> {
        let buf = self.read_fixed_bytes(len)?;
        mutf2utf(&buf)
    }
}

#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncRead + Unpin> Muncher<T> {
    /// Reads a MUTF-8 string prefixed by a length (number of bytes) of type `<E>`,
    /// then converts it to a UTF-8 [`String`].
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want raw MUTF-8, use [`Muncher::read_pref_bytes`].
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub async fn read_pref_mutf8_a<E: AsyncPrimitive>(
        &mut self,
        end: End,
    ) -> Result<String, MutfError> {
        let buf = self.read_pref_bytes_a::<E>(end).await?;
        mutf2utf(&buf)
    }

    /// Reads `len` number of bytes into a MUTF-8 string,
    /// then converts it to a UTF-8 [`String`].
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want raw MUTF-8, use [`Muncher::read_fixed_bytes`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub async fn read_fixed_mutf8_a(&mut self, len: usize) -> Result<String, MutfError> {
        let buf = self.read_fixed_bytes_a(len).await?;
        mutf2utf(&buf)
    }
}

fn mutf2utf(buf: &[u8]) -> Result<String, MutfError> {
    Ok(String::from_utf8_lossy(&mutf8::mutf8_to_utf8(buf)?).to_string())
}

fn utf2mutf(s: &str) -> Result<Cow<'_, [u8]>, MutfError> {
    Ok(mutf8::utf8_to_mutf8(s.as_bytes())?)
}

/// **MUTF-8 string and buffer related methods that require `T: `[`std::io::BufRead`]**
impl<T: BufRead> Muncher<T> {
    /// Reads a C-style string (ending with `\0` null byte)
    /// in the MUTF-8 format (tries to parse, fails if invalid).
    ///
    /// This is a niche format. For most cases, this is
    /// not what you need and you should instead use UTF-8.
    ///
    /// If you want bytes or some other format,
    /// see [`Muncher::read_cstr_bytes`].
    ///
    /// For more info on MUTF-8 see <https://crates.io/crates/mutf8>.
    pub fn read_cstr_mutf8(&mut self) -> Result<String, MutfError> {
        mutf2utf(&self.read_cstr_bytes()?)
    }
}

#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncBufRead + Unpin> Muncher<T> {
    pub async fn read_cstr_mutf8_a(&mut self) -> Result<String, MutfError> {
        mutf2utf(&self.read_cstr_bytes_a().await?)
    }
}

impl<T: Write> Muncher<T> {
    /// Writes a MUTF-8 string (converted from UTF-8)
    /// prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_pref_mutf8<P: Primitive>(
        &mut self,
        mutf: &str,
        end: End,
    ) -> Result<(), MutfError> {
        self.write_pref_bytes::<P>(end, &utf2mutf(mutf)?)?;
        Ok(())
    }

    /// Writes a MUTF-8 string (converted from UTF-8),
    /// with a null terminator.
    pub fn write_cstr_mutf8(&mut self, mutf: &str) -> Result<(), MutfError> {
        let buf = utf2mutf(mutf)?;
        self.write(&buf)?;
        if !buf.ends_with(&[0]) {
            self.write_le(0u8)?;
        }
        Ok(())
    }
}

#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncWriteExt + Unpin> Muncher<T> {
    /// Writes a MUTF-8 string (converted from UTF-8)
    /// prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn write_pref_mutf8_a<P: AsyncPrimitive>(
        &mut self,
        mutf: &str,
        end: End,
    ) -> Result<(), MutfError> {
        self.write_pref_bytes_a::<P>(end, &utf2mutf(mutf)?).await?;
        Ok(())
    }

    /// Writes a MUTF-8 string (converted from UTF-8),
    /// with a null terminator.
    pub async fn write_cstr_mutf8_a(&mut self, mutf: &str) -> Result<(), MutfError> {
        let buf = utf2mutf(mutf)?;
        self.write(&buf).await?;
        if !buf.ends_with(&[0]) {
            self.write_le_a(0u8).await?;
        }
        Ok(())
    }
}
