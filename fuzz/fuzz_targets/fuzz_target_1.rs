#![no_main]

use bytemuncher::{End, Muncher};
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut muncher = Muncher::new(Cursor::new(data));

    // Just try to read something. If this panics or crashes, fuzzing will find it.
    let _ = muncher.read_le::<u16>();
    let _ = muncher.read_be::<u32>();

    let _ = muncher.read_cstr_bytes();
    let _ = muncher.read_cstr_utf8();
    let _ = muncher.read_line_utf8();
    let _ = muncher.read_pref_utf8::<u64>(End::Little);
});
