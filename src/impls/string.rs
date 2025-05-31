use crate::{Endianness, Muncher, ReadEndian};
use std::io::{BufRead, Error, ErrorKind, Read};

/// **Size-prefixed string methods**
impl<T: Read> Muncher<T> {
    /// Reads some bytes prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_pref_bytes<E: ReadEndian>(&mut self, end: Endianness) -> Result<Vec<u8>, Error> {
        let len = self.read_m::<E>(end)?;
        let mut buf = vec![0u8; len.into_usize()];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }

    /// Reads a UTF-8 string prefixed by a length (number of bytes) of type `<E>`.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_pref_utf8<E: ReadEndian>(&mut self, end: Endianness) -> Result<String, Error> {
        String::from_utf8(self.read_pref_bytes::<E>(end)?)
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    /// Reads a UCS-2 string prefixed by a length (number of characters) in the type `<E>`.
    ///
    /// UCS-2 consists of big endian 16-bit words, each of which represent a Unicode
    /// code point between U+0000 and U+FFFF inclusive.
    ///
    /// Through the `end` argument you can choose the endianness of the length field.
    ///
    /// For more info on endianness see [`crate::Endianness`].
    pub fn read_pref_ucs2<E: ReadEndian>(&mut self, end: Endianness) -> Result<String, Error> {
        let char_count = self.read_m::<E>(end)?.into_usize();
        let mut result = String::with_capacity(char_count);

        for _ in 0..char_count {
            let code_unit = self.read_be::<u16>()?; // UCS-2 is always big-endian
            match char::from_u32(code_unit as u32) {
                Some(ch) => result.push(ch),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Invalid UCS-2 code unit: {:#X}", code_unit),
                    ));
                }
            }
        }

        Ok(result)
    }
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
        let bytes_read = self.reader.read_until(0, &mut buf)?;

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
        String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    /// Reads a line of UTF8 string (tries to parse, fails if invalid).
    /// This reads until a `\n` newline or an end-of-file is reached.
    pub fn read_line_utf8(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        self.reader.read_line(&mut buf)?;
        Ok(buf)
    }

    /// Reads bytes until the specified delimiter is encountered.
    ///
    /// Essentially it reads bytes again and again until it hits a byte that
    /// is the `delim` argument. The resulting buffer will include the delimiter
    /// byte if found.
    pub fn read_delim_bytes(&mut self, delim: u8) -> Result<Vec<u8>, Error> {
        let mut buf = Vec::new();
        self.reader.read_until(delim, &mut buf)?;
        Ok(buf)
    }
}
