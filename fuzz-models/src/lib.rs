#![allow(dead_code)]
use std::marker::PhantomData;

use strum::FromRepr;

#[allow(private_interfaces)]
#[no_mangle]
pub extern "C" fn __JUST_FOR_BINDGEN__(_: AllTypesC) {}

#[repr(u8)]
#[derive(Debug, Clone, Copy, FromRepr, Default, PartialEq, Eq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum Enum {
    #[default]
    Foo,
    Bar,
    Baz,
    Qux,
    Quux,
    Corge,
    Grault,
    Garply,
}

#[derive(Debug, Clone, Copy, FromRepr, Default)]
#[repr(u8)]
pub enum FieldKind {
    #[default]
    Void,
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Text,
    Data,
    Struct,
    Enum,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct OptionalVoid;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
struct OptionalVoidList {
    pub present: bool,
    pub length: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OptionalList<T> {
    pub present: bool,
    pub field_size: u64,
    pub item_count: u64,
    pub kind: FieldKind,
    pub start: *const T,
    pub _p: PhantomData<T>,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Text {
    pub size: u64,
    pub start: *const u8,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Data {
    pub size: u64,
    pub start: *const u8,
}

#[derive(Debug)]
#[repr(C)]
struct Optional<T: ?Sized> {
    pub kind: FieldKind,
    pub present: bool,
    pub value: T,
}

#[derive(Debug)]
#[repr(C)]
struct BoxedAllTypes {
    pub value: *const AllTypesC,
}

/// Rust representation of the all-types capnp struct to use for building from
/// arbitrary data
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct AllTypes {
    pub bool_field: bool,
    pub int8_field: i8,
    pub int16_field: i16,
    pub int32_field: i32,
    pub int64_field: i64,
    pub uint8_field: u8,
    pub uint16_field: u16,
    pub uint32_field: u32,
    pub uint64_field: u64,
    pub float32_field: f32,
    pub float64_field: f64,
    pub enum_field: Enum,
    pub text_field: Option<String>,
    pub data_field: Option<Vec<u8>>,
    pub struct_field: Option<Box<AllTypes>>,
    // interfaceField @16 : Void;  # TODO
    pub void_list: Option<Vec<()>>,
    pub bool_list: Option<Vec<bool>>,
    pub int8_list: Option<Vec<i8>>,
    pub int16_list: Option<Vec<i16>>,
    pub int32_list: Option<Vec<i32>>,
    pub int64_list: Option<Vec<i64>>,
    pub u_int8_list: Option<Vec<u8>>,
    pub u_int16_list: Option<Vec<u16>>,
    pub u_int32_list: Option<Vec<u32>>,
    pub u_int64_list: Option<Vec<u64>>,
    pub float32_list: Option<Vec<f32>>,
    pub float64_list: Option<Vec<f64>>,
    pub text_list: Option<Vec<String>>,
    pub data_list: Option<Vec<Vec<u8>>>,
    pub struct_list: Option<Vec<AllTypes>>,
    pub enum_list: Option<Vec<Enum>>,
    // interfaceList @33 : List(Void);  # TODO
}

#[derive(Debug)]
#[repr(C)]
struct AllTypesC {
    bool_field: bool,
    int8_field: i8,
    int16_field: i16,
    int32_field: i32,
    int64_field: i64,
    uint8_field: u8,
    uint16_field: u16,
    uint32_field: u32,
    uint64_field: u64,
    float32_field: f32,
    float64_field: f64,
    enum_field: Enum,
    text_field: Optional<Text>,
    data_field: Optional<Data>,
    struct_field: Optional<BoxedAllTypes>,
    // interfaceField @16 : Void;  # TODO
    void_list: OptionalVoidList,
    bool_list: OptionalList<bool>,
    int8_list: OptionalList<i8>,
    int16_list: OptionalList<i16>,
    int32_list: OptionalList<i32>,
    int64_list: OptionalList<i64>,
    u_int8_list: OptionalList<u8>,
    u_int16_list: OptionalList<u16>,
    u_int32_list: OptionalList<u32>,
    u_int64_list: OptionalList<u64>,
    float32_list: OptionalList<f32>,
    float64_list: OptionalList<f64>,
    text_list: OptionalList<Text>,
    data_list: OptionalList<Data>,
    struct_list: OptionalList<AllTypesC>,
    enum_list: OptionalList<Enum>,
    // interfaceList @33 : List(Void);  # TODO
}
