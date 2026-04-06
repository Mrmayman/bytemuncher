use crate::{End, Muncher, Primitive};
use std::io::{BufRead, Error, ErrorKind, Read, Write};

#[cfg(feature = "futures")]
use futures::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

#[cfg(any(feature = "tokio", feature = "futures"))]
use crate::traits::AsyncPrimitive;

/// **Size-prefixed string read methods** (blocking)
impl<T: Read> Muncher<T> {
    /// Reads some bytes prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_pref_bytes<E: Primitive>(&mut self, end: End) -> Result<Vec<u8>, Error> {
        let len = self.read_m::<E>(end)?.into_usize();
        self.read_fixed_bytes(len)
    }

    /// Reads `len` number of bytes into a `Vec<u8>`
    pub fn read_fixed_bytes(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        self.verify_len(len)?;
        let mut buf = vec![0u8; len];
        self.inner.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads a UTF-8 string prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn read_pref_utf8<E: Primitive>(&mut self, end: End) -> Result<String, Error> {
        bytes2utf8(self.read_pref_bytes::<E>(end)?)
    }

    /// Reads `len` number of bytes into a UTF-8 [`String`].
    pub fn read_fixed_utf8(&mut self, len: usize) -> Result<String, Error> {
        bytes2utf8(self.read_fixed_bytes(len)?)
    }

    /// Reads a UCS-2 string prefixed by a length (number of characters) in the type `<E>`.
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    #[cfg(feature = "ucs2")]
    pub fn read_pref_ucs2<E: Primitive>(&mut self, end: End) -> Result<String, Error> {
        let char_count = self.read_m::<E>(end)?.into_usize();
        self.read_fixed_ucs2(char_count)
    }

    /// Reads `char_count` number of 16-bit characters as a UCS-2 string,
    /// and converts it to UTF-8 [`String`].
    #[cfg(feature = "ucs2")]
    pub fn read_fixed_ucs2(&mut self, char_count: usize) -> Result<String, Error> {
        self.verify_len(char_count * 2)?;
        let mut result = Vec::with_capacity(char_count);

        let mut chars: Vec<u16> = Vec::new();
        for _ in 0..char_count {
            let ch = self.read_be()?;
            chars.push(ch);
        }

        ucs2::decode_with(&chars, |out| Ok(result.extend(out))).map_err(usc2err)?;
        bytes2utf8(result)
    }
}

/// **Size-prefixed string read methods** (async)
#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncReadExt + Unpin> Muncher<T> {
    /// Reads some bytes prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn read_pref_bytes_a<E: AsyncPrimitive>(
        &mut self,
        end: End,
    ) -> Result<Vec<u8>, Error> {
        let len = self.read_m_a::<E>(end).await?.into_usize();
        self.read_fixed_bytes_a(len).await
    }

    /// Reads `len` number of bytes into a `Vec<u8>`
    pub async fn read_fixed_bytes_a(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        self.verify_len(len)?;
        let mut buf = vec![0u8; len];
        self.inner.read_exact(&mut buf).await?;
        Ok(buf)
    }

    /// Reads a UTF-8 string prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn read_pref_utf8_a<E: AsyncPrimitive>(&mut self, end: End) -> Result<String, Error> {
        bytes2utf8(self.read_pref_bytes_a::<E>(end).await?)
    }

    /// Reads `len` number of bytes into a UTF-8 [`String`].
    pub async fn read_fixed_utf8_a(&mut self, len: usize) -> Result<String, Error> {
        bytes2utf8(self.read_fixed_bytes_a(len).await?)
    }

    /// Reads a UCS-2 string prefixed by a length (number of characters) in the type `<E>`.
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    #[cfg(feature = "ucs2")]
    pub async fn read_pref_ucs2_a<E: AsyncPrimitive>(&mut self, end: End) -> Result<String, Error> {
        let char_count = self.read_m_a::<E>(end).await?.into_usize();
        self.read_fixed_ucs2_a(char_count).await
    }

    /// Reads `char_count` number of 16-bit characters as a UCS-2 string,
    /// and converts it to UTF-8 [`String`].
    #[cfg(feature = "ucs2")]
    pub async fn read_fixed_ucs2_a(&mut self, char_count: usize) -> Result<String, Error> {
        self.verify_len(char_count * 2)?;
        let mut result = Vec::with_capacity(char_count);

        let mut chars: Vec<u16> = Vec::new();
        for _ in 0..char_count {
            let ch = self.read_be_a().await?;
            chars.push(ch);
        }

        ucs2::decode_with(&chars, |out| Ok(result.extend(out))).map_err(usc2err)?;
        bytes2utf8(result)
    }
}

#[cfg(feature = "ucs2")]
fn usc2err(n: ucs2::Error) -> Error {
    Error::new(ErrorKind::InvalidData, n.to_string())
}

/// **String and buffer related methods that require `T: `[`std::io::BufRead`]** (blocking)
impl<T: BufRead> Muncher<T> {
    /// Reads a C-style string (ending with `\0` null byte)
    /// into a buffer of bytes (`Vec<u8>`).
    ///
    /// **The resulting buffer does not include a null byte!
    /// Add one on your own if necessary!**
    ///
    /// If you want the UTF-8 format (regular Rust string),
    /// see [`Muncher::read_cstr_utf8`].
    pub fn read_cstr_bytes(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        let bytes_read = self.inner.read_until(0, &mut buf)?;

        if bytes_read == 0 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "EOF reached before null terminator",
            ));
        }

        // Remove the null terminator
        if buf.last() == Some(&0) {
            buf.pop();
        }
        Ok(buf)
    }

    /// Reads a C-style string (ending with `\0` null byte)
    /// in the UTF-8 format (tries to parse, fails if invalid).
    ///
    /// If you want bytes or some other format,
    /// see [`Muncher::read_cstr_bytes`].
    pub fn read_cstr_utf8(&mut self) -> Result<String, Error> {
        let buf = self.read_cstr_bytes()?;
        bytes2utf8(buf)
    }

    /// Reads bytes until the specified delimiter is encountered.
    ///
    /// Essentially it reads bytes again and again until it hits a byte that
    /// is the `delim` argument. The resulting buffer will include the delimiter
    /// byte if found.
    pub fn read_delim_bytes(&mut self, delim: u8) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        self.inner.read_until(delim, &mut buf)?;
        Ok(buf)
    }
}

/// **String and buffer related methods that require `T: `AsyncBufRead`** (async)
#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncBufReadExt + Unpin> Muncher<T> {
    /// Reads a C-style string (ending with `\0` null byte)
    /// into a buffer of bytes (`Vec<u8>`).
    ///
    /// **The resulting buffer does not include a null byte!
    /// Add one on your own if necessary!**
    ///
    /// If you want the UTF-8 format (unicode, extension of ASCII, widely used),
    /// see [`Muncher::read_cstr_utf8`].
    pub async fn read_cstr_bytes_a(&mut self) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        let bytes_read = self.inner.read_until(0, &mut buf).await?;

        if bytes_read == 0 {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                "EOF reached before null terminator",
            ));
        }

        // Remove the null terminator
        if buf.last() == Some(&0) {
            buf.pop();
        }
        Ok(buf)
    }

    /// Reads a C-style string (ending with `\0` null byte)
    /// in the UTF-8 format (tries to parse, fails if invalid).
    ///
    /// If you want bytes or some other format,
    /// see [`Muncher::read_cstr_bytes`].
    pub async fn read_cstr_utf8_a(&mut self) -> Result<String, Error> {
        let buf = self.read_cstr_bytes_a().await?;
        bytes2utf8(buf)
    }

    /// Reads bytes until the specified delimiter is encountered.
    ///
    /// Essentially it reads bytes again and again until it hits a byte that
    /// is the `delim` argument. The resulting buffer will include the delimiter
    /// byte if found.
    pub async fn read_delim_bytes_a(&mut self, delim: u8) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        self.inner.read_until(delim, &mut buf).await?;
        Ok(buf)
    }
}

fn bytes2utf8(bytes: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

// ================================
// WRITING
// ================================

/// **Size-prefixed string methods** (blocking)
impl<T: Write> Muncher<T> {
    /// Writes a fixed array of `u16`'s in the specified endianness
    /// (see [`crate::End`]).
    pub fn write_fixed_u16(&mut self, b: &[u16], end: End) -> Result<(), Error> {
        for c in b {
            self.write_m(*c, end)?;
        }
        Ok(())
    }

    /// Writes some bytes prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_pref_bytes<E: Primitive + From<usize>>(
        &mut self,
        end: End,
        buf: &[u8],
    ) -> Result<(), Error> {
        self.write_m::<E>(E::from(buf.len()), end)?;
        self.write(buf)?;
        Ok(())
    }

    /// Writes some bytes, with a null terminator.
    pub fn write_cstr_bytes(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.write(buf)?;
        if !buf.ends_with(&[0]) {
            self.write_le(0u8)?;
        }
        Ok(())
    }

    /// Writes a UCS-2 string prefixed by a length (number of characters) in the type `<E>`.
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    #[cfg(feature = "ucs2")]
    pub fn write_pref_ucs2<E: Primitive + From<usize>>(
        &mut self,
        end: End,
        msg: &str,
    ) -> Result<(), Error> {
        let mut out = Vec::new();
        ucs2::encode_with(msg, |n| Ok(out.push(n))).map_err(usc2err)?;
        self.write_m::<E>(E::from(out.len()), end)?;
        self.write_fixed_u16(&out, End::Big)?;
        Ok(())
    }
}

/// **Size-prefixed string methods** (async)
#[cfg(any(feature = "tokio", feature = "futures"))]
impl<T: AsyncWriteExt + Unpin> Muncher<T> {
    /// Writes a fixed array of `u16`'s as big endian.
    pub async fn write_fixed_u16_a(&mut self, b: &[u16], end: End) -> Result<(), Error> {
        for c in b {
            self.write_m_a(*c, end).await?;
        }
        Ok(())
    }

    /// Writes some bytes prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub async fn write_pref_bytes_a<E: AsyncPrimitive + From<usize>>(
        &mut self,
        end: End,
        buf: &[u8],
    ) -> Result<(), Error> {
        self.write_m_a::<E>(E::from(buf.len()), end).await?;
        self.write(buf).await?;
        Ok(())
    }

    /// Writes some bytes, with a null terminator.
    pub async fn write_cstr_bytes_a(&mut self, buf: &[u8]) -> Result<(), Error> {
        self.write(buf).await?;
        if !buf.ends_with(&[0]) {
            self.write_le_a(0u8).await?;
        }
        Ok(())
    }

    /// Writes a UCS-2 string prefixed by a length (number of characters) in the type `<E>`.
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    #[cfg(feature = "ucs2")]
    pub async fn write_pref_ucs2_a<E: AsyncPrimitive + From<usize>>(
        &mut self,
        end: End,
        msg: &str,
    ) -> Result<(), Error> {
        let mut out = Vec::new();
        ucs2::encode_with(msg, |n| Ok(out.push(n))).map_err(usc2err)?;
        self.write_m_a::<E>(E::from(out.len()), end).await?;
        self.write_fixed_u16_a(&out, End::Big).await?;
        Ok(())
    }
}
