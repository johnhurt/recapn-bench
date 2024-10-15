use cpp_interop::serialize_all_types as serialize_all_types_cpp;
use eyre::Result;
use fuzz_models::AllTypes;
use rust_old_capnp::{
    read_all_types as read_all_types_old_rust, serialize_all_types as serialize_all_types_old_rust,
};
use rust_recapn::all_types::{
    read_all_types as read_all_types_recapn, serialize_all_types as serialize_all_types_recapn,
};
use std::{alloc::System, time::SystemTime};

fn assert_f64_eq(left: f64, right: f64) {
    if left.is_normal() {
        assert_eq!(left, right);
    }
    if left.is_infinite() {
        assert!(right.is_infinite());
        assert_eq!(left.is_sign_negative(), right.is_sign_negative());
        assert_eq!(left.is_sign_positive(), right.is_sign_positive());
    }
    if left.is_subnormal() {
        assert!(right.is_subnormal());
        assert_eq!(left.is_sign_negative(), right.is_sign_negative());
        assert_eq!(left.is_sign_positive(), right.is_sign_positive());
    }
    if left.is_nan() {
        assert!(right.is_nan());
    }
}

fn assert_f32_eq(left: f32, right: f32) {
    if left.is_normal() {
        assert_eq!(left, right);
    }
    if left.is_infinite() {
        assert!(right.is_infinite());
        assert_eq!(left.is_sign_negative(), right.is_sign_negative());
        assert_eq!(left.is_sign_positive(), right.is_sign_positive());
    }
    if left.is_subnormal() {
        assert!(right.is_subnormal());
        assert_eq!(left.is_sign_negative(), right.is_sign_negative());
        assert_eq!(left.is_sign_positive(), right.is_sign_positive());
    }
    if left.is_nan() {
        assert!(right.is_nan());
    }
}

fn validate_old_rust_deserialize(original: &AllTypes, buffer: &[u8], packed: bool) -> Result<()> {
    let from_old_rust = read_all_types_old_rust(&buffer, packed)?;
    assert_eq!(original.bool_field, from_old_rust.bool_field);
    assert_eq!(original.int8_field, from_old_rust.int8_field);
    assert_eq!(original.int16_field, from_old_rust.int16_field);
    assert_eq!(original.int32_field, from_old_rust.int32_field);
    assert_eq!(original.int64_field, from_old_rust.int64_field);
    assert_eq!(original.uint8_field, from_old_rust.uint8_field);
    assert_eq!(original.uint16_field, from_old_rust.uint16_field);
    assert_eq!(original.uint32_field, from_old_rust.uint32_field);
    assert_eq!(original.uint64_field, from_old_rust.uint64_field);
    assert_f32_eq(original.float32_field, from_old_rust.float32_field);
    assert_f64_eq(original.float64_field, from_old_rust.float64_field);
    assert_eq!(original.enum_field, from_old_rust.enum_field);

    Ok(())
}

fn validate_recapn_deserialize(original: &AllTypes, buffer: &[u8], packed: bool) -> Result<()> {
    let from_recapn = read_all_types_recapn(&buffer, packed)?;
    assert_eq!(original.bool_field, from_recapn.bool_field);
    assert_eq!(original.int8_field, from_recapn.int8_field);
    assert_eq!(original.int16_field, from_recapn.int16_field);
    assert_eq!(original.int32_field, from_recapn.int32_field);
    assert_eq!(original.int64_field, from_recapn.int64_field);
    assert_eq!(original.uint8_field, from_recapn.uint8_field);
    assert_eq!(original.uint16_field, from_recapn.uint16_field);
    assert_eq!(original.uint32_field, from_recapn.uint32_field);
    assert_eq!(original.uint64_field, from_recapn.uint64_field);
    assert_f32_eq(original.float32_field, from_recapn.float32_field);
    assert_f64_eq(original.float64_field, from_recapn.float64_field);
    assert_eq!(original.enum_field, from_recapn.enum_field);

    Ok(())
}

/// validate the given instance of all-types to ensure that all capnp
/// implementations behave the same
pub fn validate_all_types(data: &AllTypes, valid: bool, packed: bool) -> Result<()> {
    let cpp_buffer = serialize_all_types_cpp(data, packed);
    let old_rust_buffer = serialize_all_types_old_rust(data, packed)?;
    let recapn_buffer = serialize_all_types_recapn(data, packed)?;

    assert_eq!(cpp_buffer.as_ref(), &old_rust_buffer);
    assert_eq!(cpp_buffer.as_ref(), &recapn_buffer);

    validate_old_rust_deserialize(data, &cpp_buffer, packed);
    validate_recapn_deserialize(data, &old_rust_buffer, packed);

    Ok(())
}
