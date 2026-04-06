use crate::{End, Muncher};
use std::io::Cursor;

#[test]
fn test_f32_read_write_le() {
    let original = 3.1415927f32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f32 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f32_read_write_be() {
    let original = 3.1415927f32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f32 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f32_read_write_ne() {
    let original = 3.1415927f32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_ne(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f32 = reader.read_ne().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f32_read_write_mixed_endian() {
    let original = 3.1415927f32;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_m(original, End::Little).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f32 = reader.read_m(End::Little).unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f64_read_write_le() {
    let original = 2.718281828459045f64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_le(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f64 = reader.read_le().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f64_read_write_be() {
    let original = 2.718281828459045f64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_be(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f64 = reader.read_be().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f64_read_write_ne() {
    let original = 2.718281828459045f64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_ne(original).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f64 = reader.read_ne().unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f64_read_write_mixed_endian() {
    let original = 2.718281828459045f64;
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    writer.write_m(original, End::Big).unwrap();

    let mut reader = Muncher::new(Cursor::new(buffer));
    let result: f64 = reader.read_m(End::Big).unwrap();

    assert_eq!(original, result);
}

#[test]
fn test_f32_special_values() {
    let special_values = [
        0.0f32,
        -0.0f32,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::MIN,
        f32::MAX,
        f32::EPSILON,
    ];

    for &value in &special_values {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(value).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: f32 = reader.read_le().unwrap();

        if value.is_nan() {
            assert!(result.is_nan());
        } else {
            assert_eq!(value, result);
        }
    }
}

#[test]
fn test_f64_special_values() {
    let special_values = [
        0.0f64,
        -0.0f64,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::MIN,
        f64::MAX,
        f64::EPSILON,
    ];

    for &value in &special_values {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_be(value).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: f64 = reader.read_be().unwrap();

        if value.is_nan() {
            assert!(result.is_nan());
        } else {
            assert_eq!(value, result);
        }
    }
}

#[test]
fn test_f32_nan_values() {
    let nan_values = [f32::NAN, f32::NAN];

    for &value in &nan_values {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_le(value).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: f32 = reader.read_le().unwrap();

        assert!(result.is_nan());
    }
}

#[test]
fn test_f64_nan_values() {
    let nan_values = [f64::NAN, f64::NAN];

    for &value in &nan_values {
        let mut buffer = Vec::new();
        let mut writer = Muncher::new(&mut buffer);
        writer.write_be(value).unwrap();

        let mut reader = Muncher::new(Cursor::new(buffer));
        let result: f64 = reader.read_be().unwrap();

        assert!(result.is_nan());
    }
}

#[test]
fn test_f32_endian_consistency() {
    let value = 123.456f32;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    if cfg!(target_endian = "little") {
        assert_eq!(buffer_le, value.to_le_bytes().to_vec());
        assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    } else {
        assert_eq!(buffer_le, value.to_le_bytes().to_vec());
        assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    }

    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_f64_endian_consistency() {
    let value = 123456.789f64;

    let mut buffer_le = Vec::new();
    let mut writer_le = Muncher::new(&mut buffer_le);
    writer_le.write_le(value).unwrap();

    let mut buffer_be = Vec::new();
    let mut writer_be = Muncher::new(&mut buffer_be);
    writer_be.write_be(value).unwrap();

    if cfg!(target_endian = "little") {
        assert_eq!(buffer_le, value.to_le_bytes().to_vec());
        assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    } else {
        assert_eq!(buffer_le, value.to_le_bytes().to_vec());
        assert_eq!(buffer_be, value.to_be_bytes().to_vec());
    }

    assert_ne!(buffer_le, buffer_be);
}

#[test]
fn test_f32_multiple_values() {
    let values = [1.0f32, 2.0, 3.5, -4.25, 0.125];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    for &value in &values {
        writer.write_le(value).unwrap();
    }

    let mut reader = Muncher::new(Cursor::new(buffer));
    for &expected in &values {
        let result: f32 = reader.read_le().unwrap();
        assert_eq!(expected, result);
    }
}

#[test]
fn test_f64_multiple_values() {
    let values = [1.0f64, 2.0, 3.5, -4.25, 0.125];
    let mut buffer = Vec::new();
    let mut writer = Muncher::new(&mut buffer);

    for &value in &values {
        writer.write_be(value).unwrap();
    }

    let mut reader = Muncher::new(Cursor::new(buffer));
    for &expected in &values {
        let result: f64 = reader.read_be().unwrap();
        assert_eq!(expected, result);
    }
}

#[test]
fn test_f32_insufficient_data() {
    let buffer = [0x00, 0x00, 0x00]; // Only 3 bytes, f32 needs 4
    let mut reader = Muncher::new(Cursor::new(buffer));

    let result: Result<f32, _> = reader.read_le();
    assert!(result.is_err());
}

#[test]
fn test_f64_insufficient_data() {
    let buffer = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]; // Only 7 bytes, f64 needs 8
    let mut reader = Muncher::new(Cursor::new(buffer));

    let result: Result<f64, _> = reader.read_le();
    assert!(result.is_err());
}
