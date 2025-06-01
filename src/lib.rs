#![doc = include_str!("../README.md")]

mod impls;
mod io_read;
#[cfg(feature = "mutf8")]
mod mutf;
mod traits;

/// A helpful wrapper around any [`std::io::Read`] type.
///
/// You can put in any [`std::io::Read`] type and get access to the additional
/// bytemuncher methods for it, such as:
/// - Reading various signed/unsigned integer types in various endianness (see [`End`]).
/// - Reading floating point values in various endianness.
/// - Reading strings in various formats (UTF-8, MUTF-8, UCS-2, raw bytes)
//    from various storage types (Null terminated, length prefix, newline, ...)
///
/// # Example
/// ```
/// use bytemuncher::Muncher;
/// use std::io::Cursor;
///
/// let data = [
///     0x34, 0x12, 0x56, 0x78,
///     0x00, 0x01,
/// ];
/// let mut muncher = Muncher::new(Cursor::new(data));
///
/// assert_eq!(muncher.read_le::<u16>().unwrap(), 0x1234);
/// assert_eq!(muncher.read_be::<u16>().unwrap(), 0x5678);
///
/// assert_eq!(muncher.read_le::<u8>().unwrap(), 0);
/// assert_eq!(muncher.read_le::<u8>().unwrap(), 1);
///
/// // End of data
/// assert!(muncher.read_le::<u8>().is_err());
/// ```
///
/// For more examples see the crate documentation.
///
/// # Errors
/// All methods return [`std::io::Error`] on failure, except
/// for MUTF-8 methods (crate feature: `mutf8`) which return `MutfError`
/// on failure.
///
/// They can come from:
/// - I/O errors from the underlying reader
/// - Unexpected end-of-input
/// - Invalid encoding (e.g., malformed UTF-8 or MUTF-8)
///
/// # Note on strings
/// The string-related methods are named in the
/// `read_<format>_<encoding>[_destination]` scheme.
///
/// ## Format
/// Specifies how the size of the string is found.
/// It can be one of:
/// - `size8`/`size16`: A u8 or u16 prefix before the
///   contents indicating the length of the string.
/// - `cstr`: The string ends with a null `\0` byte.
/// - `line` (`BufRead` only): Reads till a newline or end-of-file is met.
/// - `delim` (`BufRead` only): Reads till the byte specified in the argument
///   is met or the input has ended, and returns a string containing the byte if found.
///
/// ## Encoding
/// Specifies the character encoding of the string.
/// It can be one of:
/// - `bytes`: Returns a raw `Vec<u8>` containing the bytes,
///   feel free to interpret as you wish.
/// - `utf8`: Reads and returns a UTF-8 encoded [`String`]
///   (tries to parse, fails if invalid). This is the default
///   rust type of string.
/// - `mutf8`: (with crate feature `mutf8`)
///   Reads a MUTF-8 encoded string and converts it to
///   a UTF-8 [`String`]. This is usually not used unless
///   you specifically need MUTF-8 support.
/// - `ucs2`: Reads as the UCS-2 string format (explained below),
///   and converts it to a UTF-8 [`String`].
///
/// UCS-2 consists of big endian 16-bit words, each of which represent
/// a Unicode code point between U+0000 and U+FFFF inclusive.
///
/// ## Destination (optional)
/// Specifies where to store the data to. If it's not specified then
/// that usually means a new heap-allocated buffer ([`String`] or [`Vec`])
/// is made. Otherwise:
/// - `to`: (TODO) Writes to a `&mut String` or `&mut Vec`
/// - `to_exact`: (TODO) Writes to any `&mut [u8]`,
///   returns `bool` if it fit in or not.
///
/// More destinations may come in future versions.
pub struct Muncher<T> {
    reader: T,
    alloc_limit_bytes: usize,
}

impl<T> Muncher<T> {
    /// Creates a new [`Muncher`] with the default configuration:
    /// - Allocation limit of 1 GB [`Muncher::set_max_alloc`]
    /// More coming in the future.
    ///
    /// If you want to iterate over some `[u8]`
    /// then use
    /// ```no_run
    /// # use bytemuncher::Muncher;
    /// # let SOMETHING = [0];
    /// # _ =
    /// Muncher::new(std::io::Cursor::new(SOMETHING))
    /// # ;
    /// ```
    pub fn new(reader: T) -> Self {
        Self {
            reader,
            alloc_limit_bytes: 1 * 1024 * 1024 * 1024,
        }
    }

    /// Sets a custom memory allocation limit (in bytes) for the [`Muncher`].
    /// This limits how much memory can be allocated when reading bytes.
    ///
    /// Default: 1 GB
    ///
    /// This prevents the scenario where if you take in garbage data
    /// and interpret it as the length, this could allocate terabytes of memory
    /// and crash.
    pub fn set_max_alloc(&mut self, alloc_limit_bytes: usize) -> &mut Self {
        self.alloc_limit_bytes = alloc_limit_bytes;
        self
    }
}

/// The endianness of a value.
///
/// Endianness represents the order
/// in which a number may be stored as bytes.
///
/// For example, the number `0x12345678` may be
/// stored in memory as:
/// - `0x12, 0x34, 0x56, 0x78` in big endian
/// - `0x78, 0x56, 0x34, 0x12` in little endian
///
/// Big endian is commonly found in network packets
/// and some file formats.
///
/// Little endian is the "native" format for most CPUs,
/// and some file formats use it.
///
/// Here, If you use native endian, the default endianness
/// (little/big) of your computer's CPU will be used.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    /// In little endian, the most significant byte appears
    /// as the last byte. You can think of this as an
    /// **opposite** of the "human-readable" order.
    ///
    /// the number `0x12345678` may be stored in memory as:
    /// `0x78, 0x56, 0x34, 0x12` in little endian
    ///
    /// Little endian is the "native" format for most CPUs,
    /// and some file formats use it.
    Little,
    /// In big endian, the most significant byte appears
    /// as the first byte. You can think of this as a
    /// "human-readable" order.
    ///
    /// the number `0x12345678` may be stored in memory as:
    /// `0x12, 0x34, 0x56, 0x78` in big endian
    ///
    /// Big endian is commonly found in network packets
    /// and some file formats.
    Big,
    /// Here, "Native" endian refers to the default endianness
    /// (little/big) of your computer's CPU will be used.
    ///
    /// So either Little (likely) or Big (unlikely) will
    /// automatically be picked based on your computer.
    ///
    /// If you want to check whether your CPU is little
    /// endian or not, use the [`IS_TARGET_LITTLE_ENDIAN`]
    /// constant.
    Native,
}

impl End {
    /// Checks whether the value is little endian or not.
    ///
    /// Automatically works for [`End::Native`] so this could
    /// be useful.
    pub fn is_le(self) -> bool {
        match self {
            End::Little => true,
            End::Big => false,
            End::Native => IS_TARGET_LITTLE_ENDIAN,
        }
    }

    /// Returns the "opposite" endianness of `self`;
    /// ie. it flips Little to Big and vice versa.
    ///
    /// It automatically works with [`End::Native`].
    pub fn opposite(self) -> Self {
        match self {
            End::Little => End::Big,
            End::Big => End::Little,
            End::Native => {
                if IS_TARGET_LITTLE_ENDIAN {
                    End::Big
                } else {
                    End::Little
                }
            }
        }
    }

    /// Checks whether the given endianness is
    /// equal to your CPU's native endianness.
    pub fn is_target_endian(self) -> bool {
        match self {
            End::Little => IS_TARGET_LITTLE_ENDIAN,
            End::Big => !IS_TARGET_LITTLE_ENDIAN,
            End::Native => true,
        }
    }
}

#[cfg(target_endian = "little")]
pub const IS_TARGET_LITTLE_ENDIAN: bool = true;
#[cfg(target_endian = "big")]
pub const IS_TARGET_LITTLE_ENDIAN: bool = false;

#[cfg(feature = "mutf8")]
pub use mutf::MutfError;
/// Important exports of the `mutf8` crate
#[cfg(feature = "mutf8")]
pub mod mutf_8 {
    pub use mutf8::error::Error as MutfInnerError;
    pub use mutf8::error::{Expected, Mode, Position};
    pub use mutf8::{mutf8_to_utf8, utf8_to_mutf8};
}

pub use traits::ReadEndian;
