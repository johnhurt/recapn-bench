use crate::bindings::{
    self, AllTypesC, BoxedAllTypes, Data, Optional, OptionalList, OptionalVoidList, Text,
};
use fuzz_models::{AllTypes, Enum, FieldKind};
use std::ptr;

impl Optional<BoxedAllTypes> {
    fn from_boxed_all_types_opt(
        other_opt: &Option<Box<AllTypes>>,
        conversions: &mut Conversions,
    ) -> Self {
        match other_opt.as_ref() {
            Some(other) => Optional {
                kind: FieldKind::Struct as u8 as u8,
                present: true,
                value: AllTypesC::from_boxed_rust_struct(other, conversions),
                _phantom_0: Default::default(),
            },
            None => Optional {
                kind: FieldKind::Struct as u8 as u8,
                present: false,
                value: BoxedAllTypes { value: ptr::null() },
                _phantom_0: Default::default(),
            },
        }
    }
}

impl Optional<Text> {
    fn from_string_opt(other_opt: &Option<String>) -> Self {
        match other_opt.as_ref() {
            Some(other) => Optional {
                kind: FieldKind::Text as u8,
                present: true,
                value: Text {
                    size: other.len() as u64,
                    start: other.as_bytes().as_ptr(),
                },
                _phantom_0: Default::default(),
            },
            None => Optional {
                kind: FieldKind::Text as u8,
                present: false,
                value: Text {
                    size: 0,
                    start: ptr::null(),
                },
                _phantom_0: Default::default(),
            },
        }
    }
}

impl Optional<Data> {
    fn from_bytes_opt(other_opt: &Option<Vec<u8>>) -> Self {
        match other_opt.as_ref() {
            Some(other) => Optional {
                kind: FieldKind::Data as u8,
                present: true,
                value: Data {
                    size: other.len() as u64,
                    start: other.as_ptr(),
                },
                _phantom_0: Default::default(),
            },
            None => Optional {
                kind: FieldKind::Data as u8,
                present: false,
                value: Data {
                    size: 0,
                    start: ptr::null(),
                },
                _phantom_0: Default::default(),
            },
        }
    }
}

impl Optional<bindings::Enum> {
    fn from_enum_opt(other_opt: &Option<Enum>) -> Self {
        match other_opt.as_ref() {
            Some(other) => Optional {
                kind: FieldKind::Enum as u8,
                present: true,
                value: *other as u8,
                _phantom_0: Default::default(),
            },
            None => Optional {
                kind: FieldKind::Data as u8,
                ..Default::default()
            },
        }
    }
}

impl<'a, T: Default + Copy> From<(&'a Option<T>, FieldKind)> for Optional<T> {
    fn from((other_opt, kind): (&'a Option<T>, FieldKind)) -> Self {
        match other_opt.as_ref() {
            Some(other) => Optional {
                kind: kind as u8,
                present: true,
                value: *other,
                _phantom_0: Default::default(),
            },
            None => Optional {
                kind: kind as u8,
                present: false,
                value: T::default(),
                _phantom_0: Default::default(),
            },
        }
    }
}

impl OptionalList<Text> {
    fn from_string_list_opt(
        other_opt: &Option<Vec<String>>,
        conversions: &mut Conversions,
    ) -> Self {
        match other_opt.as_ref() {
            Some(other) => {
                let text_list = other
                    .iter()
                    .map(|s| Text {
                        size: s.len() as u64,
                        start: s.as_bytes().as_ptr(),
                    })
                    .collect::<Vec<_>>();

                let result = OptionalList {
                    kind: FieldKind::Text as u8,
                    present: true,
                    field_size: size_of::<Text>() as u64,
                    item_count: other.len() as u64,
                    start: text_list.as_ptr() as *const _,
                    ..Default::default()
                };

                conversions.text_lists.push(text_list);
                result
            }
            None => OptionalList {
                kind: FieldKind::Text as u8,
                ..Default::default()
            },
        }
    }
}

impl OptionalList<Data> {
    fn from_data_list_opt(other_opt: &Option<Vec<Vec<u8>>>, conversions: &mut Conversions) -> Self {
        match other_opt.as_ref() {
            Some(other) => {
                let data_list = other
                    .iter()
                    .map(|s| Data {
                        size: s.len() as u64,
                        start: s.as_ptr(),
                    })
                    .collect::<Vec<_>>();

                let result = OptionalList {
                    kind: FieldKind::Data as u8,
                    present: true,
                    field_size: size_of::<Data>() as u64,
                    item_count: other.len() as u64,
                    start: data_list.as_ptr() as *const _,
                    ..Default::default()
                };

                conversions.data_lists.push(data_list);
                result
            }
            None => OptionalList {
                kind: FieldKind::Data as u8,
                ..Default::default()
            },
        }
    }
}

impl OptionalList<AllTypesC> {
    fn from_struct_list_opt(
        other_opt: &Option<Vec<AllTypes>>,
        conversions: &mut Conversions,
    ) -> Self {
        match other_opt.as_ref() {
            Some(other) => {
                let struct_list = other
                    .iter()
                    .map(|s| AllTypesC::from_rust_struct_helper(s, conversions))
                    .collect::<Vec<_>>();

                let result = OptionalList {
                    kind: FieldKind::Struct as u8,
                    present: true,
                    field_size: size_of::<AllTypesC>() as u64,
                    item_count: other.len() as u64,
                    start: struct_list.as_ptr() as *const _,
                    ..Default::default()
                };

                conversions.struct_lists.push(struct_list);
                result
            }
            None => OptionalList {
                kind: FieldKind::Struct as u8,
                ..Default::default()
            },
        }
    }
}

impl OptionalList<bindings::Enum> {
    fn from_enum_list_opt(other_opt: &Option<Vec<Enum>>) -> Self {
        match other_opt.as_ref() {
            Some(other) => OptionalList {
                kind: FieldKind::Enum as u8,
                present: true,
                field_size: size_of::<Enum>() as u64,
                item_count: other.len() as u64,
                start: other.as_ptr() as *const _ as *const u8,
                ..Default::default()
            },
            None => OptionalList {
                kind: FieldKind::Struct as u8,
                ..Default::default()
            },
        }
    }
}

impl<'a, T: Default> From<(&'a Option<Vec<T>>, FieldKind)> for OptionalList<T> {
    fn from((other_opt, kind): (&'a Option<Vec<T>>, FieldKind)) -> Self {
        match other_opt.as_ref() {
            Some(other) => OptionalList {
                kind: kind as u8,
                present: true,
                field_size: size_of::<T>() as u64,
                item_count: other.len() as u64,
                start: other.as_ptr() as *const _,
                ..Default::default()
            },
            None => OptionalList {
                kind: kind as u8,
                ..Default::default()
            },
        }
    }
}

impl<T> Default for OptionalList<T> {
    fn default() -> Self {
        Self {
            field_size: 0,
            item_count: 0,
            kind: FieldKind::Void as u8,
            present: false,
            start: ptr::null(),
            _phantom_0: Default::default(),
        }
    }
}

impl Default for OptionalVoidList {
    fn default() -> Self {
        OptionalVoidList {
            length: 0,
            present: false,
        }
    }
}

impl<T: Default> Default for Optional<T> {
    fn default() -> Self {
        Optional {
            kind: 0,
            present: false,
            value: T::default(),
            _phantom_0: Default::default(),
        }
    }
}

#[allow(clippy::vec_box)]
#[derive(Default, Debug)]
pub struct Conversions {
    text_lists: Vec<Vec<Text>>,
    data_lists: Vec<Vec<Data>>,
    structs: Vec<Box<AllTypesC>>,
    struct_lists: Vec<Vec<AllTypesC>>,
}

impl AllTypesC {
    fn from_boxed_rust_struct(source: &AllTypes, conversions: &mut Conversions) -> BoxedAllTypes {
        let converted = Box::new(AllTypesC::from_rust_struct_helper(source, conversions));
        let result = BoxedAllTypes {
            value: Box::as_ref(&converted) as *const AllTypesC,
        };

        conversions.structs.push(converted);

        result
    }

    fn from_rust_struct_helper(value: &AllTypes, conversions: &mut Conversions) -> AllTypesC {
        AllTypesC {
            bool_field: value.bool_field,
            int8_field: value.int8_field,
            int16_field: value.int16_field,
            int32_field: value.int32_field,
            int64_field: value.int64_field,
            uint8_field: value.uint8_field,
            uint16_field: value.uint16_field,
            uint32_field: value.uint32_field,
            uint64_field: value.uint64_field,
            float32_field: value.float32_field,
            float64_field: value.float64_field,
            struct_field: Optional::from_boxed_all_types_opt(&value.struct_field, conversions),
            text_field: Optional::from_string_opt(&value.text_field),
            data_field: Optional::from_bytes_opt(&value.data_field),
            enum_field: value.enum_field as u8,

            void_list: value
                .void_list
                .as_ref()
                .map(|list| OptionalVoidList {
                    present: true,
                    length: list.len() as u64,
                })
                .unwrap_or_default(),
            bool_list: OptionalList::from((&value.bool_list, FieldKind::Bool)),
            int8_list: OptionalList::from((&value.int8_list, FieldKind::I8)),
            int16_list: OptionalList::from((&value.int16_list, FieldKind::I16)),
            int32_list: OptionalList::from((&value.int32_list, FieldKind::I32)),
            int64_list: OptionalList::from((&value.int64_list, FieldKind::I64)),
            u_int8_list: OptionalList::from((&value.u_int8_list, FieldKind::U8)),
            u_int16_list: OptionalList::from((&value.u_int16_list, FieldKind::U16)),
            u_int32_list: OptionalList::from((&value.u_int32_list, FieldKind::U32)),
            u_int64_list: OptionalList::from((&value.u_int64_list, FieldKind::U64)),
            float32_list: OptionalList::from((&value.float32_list, FieldKind::F32)),
            float64_list: OptionalList::from((&value.float64_list, FieldKind::F64)),
            text_list: OptionalList::from_string_list_opt(&value.text_list, conversions),
            data_list: OptionalList::from_data_list_opt(&value.data_list, conversions),
            struct_list: OptionalList::from_struct_list_opt(&value.struct_list, conversions),
            enum_list: OptionalList::from_enum_list_opt(&value.enum_list),
        }
    }

    pub fn from_rust_struct(value: &AllTypes) -> (AllTypesC, Conversions) {
        let mut conversions = Conversions::default();

        (
            AllTypesC::from_rust_struct_helper(value, &mut conversions),
            conversions,
        )
    }
}

// impl Drop for Conversions {
//     fn drop(&mut self) {
//         panic!("NO");
//     }
// }

#[cfg(test)]
mod test {
    use std::slice::from_raw_parts;

    use super::*;

    #[test]
    fn test_text_field() {
        let mut v = AllTypes::default();
        v.text_field = Some("String".to_owned());

        let (c_v, _conversions) = AllTypesC::from_rust_struct(&v);

        assert!(c_v.text_field.present);

        let Text { size, start } = c_v.text_field.value;

        let text_as_slice = unsafe { from_raw_parts(start, size as usize) };
        assert_eq!(b"String", text_as_slice);
    }

    #[test]
    fn test_byte_list() {
        let mut v = AllTypes::default();
        v.u_int8_list = Some(b"String".to_vec());

        let (c_v, _conversions) = AllTypesC::from_rust_struct(&v);

        assert!(c_v.u_int8_list.present);

        let OptionalList {
            present,
            field_size,
            item_count,
            kind,
            start,
            ..
        } = c_v.u_int8_list;

        let text_as_slice = unsafe { from_raw_parts(start, item_count as usize) };
        assert_eq!(b"String", text_as_slice);
    }
}
