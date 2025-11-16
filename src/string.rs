use crate::{End, Muncher, Primitive};
use std::io::{BufRead, Error, ErrorKind, Read, Write};

/// **Size-prefixed string read methods**
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

#[cfg(feature = "ucs2")]
fn usc2err(n: ucs2::Error) -> Error {
    Error::new(ErrorKind::InvalidData, n.to_string())
}

/// **String and buffer related methods that require `T: `[`std::io::BufRead`]**
impl<T: BufRead> Muncher<T> {
    /// Reads a C-style string (ending with `\0` null byte)
    /// into a buffer of bytes (`Vec<u8>`).
    ///
    /// **The resulting buffer does not include a null byte!
    /// Add one on your own if necessary!**
    ///
    /// If you want the UTF-8 format (unicode, extension of ASCII, widely used),
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

    /// Reads a line of UTF8 string (tries to parse, fails if invalid).
    /// This reads until a `\n` newline or an end-of-file is reached.
    pub fn read_line_utf8(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        self.inner.read_line(&mut buf)?;
        Ok(buf)
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

fn bytes2utf8(bytes: Vec<u8>) -> Result<String, Error> {
    String::from_utf8(bytes).map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

// ================================
// WRITING
// ================================

/// **Size-prefixed string methods**
impl<T: Write> Muncher<T> {
    pub fn write_fixed_u16(&mut self, ucs: &[u16]) -> Result<(), Error> {
        for c in ucs {
            self.write_be(*c)?;
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
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::End`].
    pub fn write_cstr_bytes<E: Primitive + From<usize>>(
        &mut self,
        buf: &[u8],
    ) -> Result<(), Error> {
        self.write(buf)?;
        if !buf.ends_with(&[0]) {
            self.write_le(0u8)?;
        }
        Ok(())
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
    pub fn write_pref_ucs2<E: Primitive + From<usize>>(
        &mut self,
        end: End,
        msg: &str,
    ) -> Result<(), Error> {
        let mut out = Vec::new();
        ucs2::encode_with(msg, |n| Ok(out.push(n))).map_err(usc2err)?;
        self.write_m::<E>(E::from(out.len()), end)?;
        self.write_fixed_u16(&out)?;
        Ok(())
    }
}
