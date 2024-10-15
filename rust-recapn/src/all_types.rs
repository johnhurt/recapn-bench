use std::{
    borrow::{Borrow, BorrowMut},
    io::Write,
};

use crate::gen::all_types_capnp::*;
use eyre::Result;
use fuzz_models::{AllTypes, Enum};
use recapn::arena::ReadArena;
use recapn::io::{
    read_from_slice, read_from_stream, read_packed_from_stream, PackedStream, SegmentSet,
    SegmentSetTable, StreamOptions,
};
use recapn::ty::Struct;
use recapn::{message::Message, ptr::StructBuilder};

fn from_enum(e: Enum) -> TestEnum {
    match e {
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

fn serialize_all_types_helper(
    src: &AllTypes,
    target: &mut TestAllTypes<StructBuilder<'_>>,
) -> Result<()> {
    target.bool_field().set(src.bool_field);
    target.int8_field().set(src.int8_field);
    target.int16_field().set(src.int16_field);
    target.int32_field().set(src.int32_field);
    target.int64_field().set(src.int64_field);
    target.u_int8_field().set(src.uint8_field);
    target.u_int16_field().set(src.uint16_field);
    target.u_int32_field().set(src.uint32_field);
    target.u_int64_field().set(src.uint64_field);
    target.float32_field().set(src.float32_field);
    target.float64_field().set(src.float64_field);
    target.enum_field().set(from_enum(src.enum_field));
    Ok(())
}

pub fn serialize_all_types(v: &AllTypes, packed: bool) -> Result<Vec<u8>> {
    let mut message = Message::global();
    let mut builder = message.builder().init_struct_root::<TestAllTypes>();
    let mut result = vec![];

    serialize_all_types_helper(v, &mut builder)?;

    if packed {
        recapn::io::write_message_packed(&mut result, &message.segments().unwrap()).unwrap();
    } else {
        recapn::io::write_message(&mut result, &message.segments().unwrap()).unwrap();
    }

    Ok(result)
}

pub fn read_all_types(src: &[u8], packed: bool) -> Result<AllTypes> {
    let segments;

    if packed {
        let mut message_reader = PackedStream::new(src);
        segments = read_packed_from_stream(&mut message_reader, StreamOptions::default()).unwrap();
    } else {
        segments = read_from_stream(src, StreamOptions::DEFAULT).unwrap();
    }
    let message =
        recapn::message::Reader::new(&segments, recapn::message::ReaderOptions::default());
    let at = message.read_as_struct::<TestAllTypes>();

    let mut result = AllTypes::default();
    result.bool_field = at.bool_field();
    result.int8_field = at.int8_field();
    result.int16_field = at.int16_field();
    result.int32_field = at.int32_field();
    result.int64_field = at.int64_field();
    result.uint8_field = at.u_int8_field();
    result.uint16_field = at.u_int16_field();
    result.uint32_field = at.u_int32_field();
    result.uint64_field = at.u_int64_field();
    result.float32_field = at.float32_field();
    result.float64_field = at.float64_field();
    result.enum_field = Enum::from_repr(at.enum_field().unwrap() as u8).unwrap();

    Ok(result)
}
