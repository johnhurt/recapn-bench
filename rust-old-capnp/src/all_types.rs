use capnp::{message, serialize, serialize_packed};
use eyre::{Context, Result};
use fuzz_models::{AllTypes, Enum};

use crate::all_types_capnp::{test_all_types, TestEnum};

pub fn read_all_types(src: &[u8], packed: bool) -> Result<AllTypes> {
    let message_reader;
    if packed {
        message_reader =
            serialize_packed::read_message(src, ::capnp::message::ReaderOptions::new())?;
    } else {
        message_reader = serialize::read_message(src, ::capnp::message::ReaderOptions::new())?;
    }

    let mut result = AllTypes::default();

    let at = message_reader
        .get_root::<test_all_types::Reader>()
        .wrap_err("Reader error")?;

    result.bool_field = at.get_bool_field();
    result.int8_field = at.get_int8_field();
    result.int16_field = at.get_int16_field();
    result.int32_field = at.get_int32_field();
    result.int64_field = at.get_int64_field();
    result.uint8_field = at.get_u_int8_field();
    result.uint16_field = at.get_u_int16_field();
    result.uint32_field = at.get_u_int32_field();
    result.uint64_field = at.get_u_int64_field();
    result.float32_field = at.get_float32_field();
    result.float64_field = at.get_float64_field();
    result.enum_field = Enum::from_repr(at.get_enum_field().unwrap() as u8).unwrap();

    if at.has_text_field() {
        result.text_field = Some(at.get_text_field().unwrap().to_string().unwrap())
    }

    Ok(result)
}

fn from_enum(e: Enum) -> TestEnum {
    match (e) {
        Enum::Foo => TestEnum::Foo,
        Enum::Bar => TestEnum::Bar,
        Enum::Baz => TestEnum::Baz,
        Enum::Qux => TestEnum::Qux,
        Enum::Quux => TestEnum::Quux,
        Enum::Corge => TestEnum::Corge,
        Enum::Grault => TestEnum::Grault,
        Enum::Garply => TestEnum::Garply,
    }
}

fn serialize_all_types_helper(v: &AllTypes, builder: &mut test_all_types::Builder<'_>) {
    builder.set_bool_field(v.bool_field);
    builder.set_int8_field(v.int8_field);
    builder.set_int16_field(v.int16_field);
    builder.set_int32_field(v.int32_field);
    builder.set_int64_field(v.int64_field);
    builder.set_u_int8_field(v.uint8_field);
    builder.set_u_int16_field(v.uint16_field);
    builder.set_u_int32_field(v.uint32_field);
    builder.set_u_int64_field(v.uint64_field);
    builder.set_float32_field(v.float32_field);
    builder.set_float64_field(v.float64_field);
    builder.set_enum_field(from_enum(v.enum_field));

    if let Some(text_field) = v.text_field.as_ref() {
        builder.set_text_field(text_field);
    }
}

pub fn serialize_all_types(v: &AllTypes, packed: bool) -> Result<Vec<u8>> {
    let mut target = vec![];
    let mut message = ::capnp::message::Builder::new_default();
    {
        let mut all_types = message.init_root::<test_all_types::Builder>();
        serialize_all_types_helper(v, &mut all_types);
    }

    if packed {
        serialize_packed::write_message(&mut target, &message).wrap_err("Serialization error")?;
    } else {
        serialize::write_message(&mut target, &message).wrap_err("Serialization error")?;
    }

    Ok(target)
}
