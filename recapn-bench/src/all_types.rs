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
    assert_all_types_eq(original, &from_old_rust);
    Ok(())
}

fn validate_recapn_deserialize(original: &AllTypes, buffer: &[u8], packed: bool) -> Result<()> {
    let from_recapn = read_all_types_recapn(&buffer, packed)?;
    assert_all_types_eq(original, &from_recapn);
    Ok(())
}

fn assert_all_types_eq(left: &AllTypes, right: &AllTypes) {
    assert_eq!(left.bool_field, right.bool_field);
    assert_eq!(left.int8_field, right.int8_field);
    assert_eq!(left.int16_field, right.int16_field);
    assert_eq!(left.int32_field, right.int32_field);
    assert_eq!(left.int64_field, right.int64_field);
    assert_eq!(left.uint8_field, right.uint8_field);
    assert_eq!(left.uint16_field, right.uint16_field);
    assert_eq!(left.uint32_field, right.uint32_field);
    assert_eq!(left.uint64_field, right.uint64_field);
    assert_f32_eq(left.float32_field, right.float32_field);
    assert_f64_eq(left.float64_field, right.float64_field);
    assert_eq!(left.enum_field, right.enum_field);
    assert_eq!(left.text_field, right.text_field);
    assert_eq!(left.data_field, right.data_field);
}

/// validate the given instance of all-types to ensure that all capnp
/// implementations behave the same
pub fn validate_all_types(data: &AllTypes, valid: bool, packed: bool) -> Result<()> {
    let cpp_buffer = serialize_all_types_cpp(data, packed);
    let old_rust_buffer = serialize_all_types_old_rust(data, packed)?;
    let recapn_buffer = serialize_all_types_recapn(data, packed)?;

    //assert_eq!(cpp_buffer.as_ref(), &old_rust_buffer);
    assert_eq!(cpp_buffer.as_ref(), &recapn_buffer);

    validate_old_rust_deserialize(data, &recapn_buffer, packed);
    validate_recapn_deserialize(data, &old_rust_buffer, packed);

    Ok(())
}
