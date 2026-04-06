# bytemuncher
Simple, flexible parsing helpers for [Rust](https://rust-lang.org)'s `std::io::Read`/`Write` traits.

You can wrap any [`std::io::Read`] or [`std::io::Write`] type
in the `bytemuncher::Muncher<T>` type and be able to read/write:
- Various signed/unsigned integer types in various endianness (see [`End`]).
- Floating point values in various endianness.
- Strings in various formats (UTF-8, MUTF-8, UCS-2, raw bytes)
  from various storage types (Null terminated, length prefix, newline, ...)

**For more information, see the documentation of `bytemuncher::Muncher` and its methods.**

# Example

## Numbers

```rust
use bytemuncher::Muncher;
use std::io::Cursor;

let data = [
    0x34, 0x12, 0x56, 0x78,
    0x00, 0x01,
];
let mut muncher = Muncher::new(Cursor::new(data));

// Any integer and float are supported!
assert_eq!(muncher.read_le::<u16>().unwrap(), 0x1234);
assert_eq!(muncher.read_be::<u16>().unwrap(), 0x5678);

assert_eq!(muncher.read_le::<u8>().unwrap(), 0);
assert_eq!(muncher.read_le::<u8>().unwrap(), 1);

// End of data
assert!(muncher.read_le::<u8>().is_err());
```

## Strings

```rust
use bytemuncher::Muncher;
use std::io::Cursor;

let data = b"hello world\0goodbye world";
let mut muncher = Muncher::new(Cursor::new(data));

assert_eq!(muncher.read_cstr_utf8().unwrap(), "hello world");
assert_eq!(muncher.read_cstr_utf8().unwrap(), "goodbye world");
```

You can also run the test suite by doing `cargo test`

---

# Async

This crate supports async, either through crate feature
`tokio` or `futures` (**not both**). All your methods are there,
just add `_a` to the end of their names.

It may use `AsyncRead`, `AsyncWrite`, and `AsyncBufRead`.

# Why not [byteorder](https://crates.io/crate/byteorder)?

`byteorder` and `bytemuncher` have similar, but slightly divergent goals.

- This crate helps with many kinds of string parsing, byteorder does not
- This crate has a particular focus on file parsing and network operations, and is helpful for that specifically. Hence it supports many string formats as well as (in the future) async I/O
- In general, byteorder is generic for many use cases, this is more specific in what it wants to be

# Extensibility

You must be wondering, why was this implemented as a `Muncher` struct
instead of some kind of `ReadExt`/`WriteExt` trait. This design favors library
simplicity and ease of use

Bytemuncher still remains extensible. You can create your own extra
trait, implement it for Bytemuncher, and use `Muncher`'s public
implementation of `std::io::Read`/`Write` to do your own parsing.

(although if you have any novel data types, feel free to make a PR!
I'm always happy to merge!)

```rust
use std::io::{Read, Write};
use bytemuncher::Muncher;

struct Vec2 {
    x: f32,
    y: f32,
}

pub trait ParseVec2 {
    fn read_vec2(&mut self) -> Result<Vec2, std::io::Error>;
    fn write_vec2(&mut self, val: Vec2) -> std::io::Result<()>;
}

// Having both read and write are optional,
// feel free to just have one
impl<T: Read + Write> ParseVec2 for Muncher<T> {
    fn read_vec2(&mut self) -> std::io::Result<Vec2> {
        let x = self.read_le::<f32>()?;
        let y = self.read_le::<f32>()?;

        // you can also call the underlying I/O traits

        // let buf = Vec::new();
        // self.read(&mut buf)?;

        // or use some other Read method
        // you can even use BufRead if T implements it

        Ok(Vec2 { x, y })
    }
    
    fn write_vec2(&mut self, val: Vec2) -> std::io::Result<()> {
        self.write_le(val.x)?;
        self.write_le(val.y)?;
        Ok(())
    }
}
```

# TODO

- [ ] Add destinations for reading strings into
- [ ] Optional rewind/seek feature for any io type
