#pragma once
struct AllTypesC;
#include <cstdint>

enum class Enum : uint8_t {
  Foo,
  Bar,
  Baz,
  Qux,
  Quux,
  Corge,
  Grault,
  Garply,
};

enum class FieldKind : uint8_t {
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
};

struct Text {
  uint64_t size;
  const uint8_t *start;
};

template<typename T>
struct Optional {
  FieldKind kind;
  bool present;
  T value;
};

struct Data {
  uint64_t size;
  const uint8_t *start;
};

struct BoxedAllTypes {
  const AllTypesC *value;
};

struct OptionalVoidList {
  bool present;
  uint64_t length;
};

template<typename T>
struct OptionalList {
  bool present;
  uint64_t field_size;
  uint64_t item_count;
  FieldKind kind;
  const T *start;
};

struct AllTypesC {
  bool bool_field;
  int8_t int8_field;
  int16_t int16_field;
  int32_t int32_field;
  int64_t int64_field;
  uint8_t uint8_field;
  uint16_t uint16_field;
  uint32_t uint32_field;
  uint64_t uint64_field;
  float float32_field;
  double float64_field;
  Enum enum_field;
  Optional<Text> text_field;
  Optional<Data> data_field;
  Optional<BoxedAllTypes> struct_field;
  OptionalVoidList void_list;
  OptionalList<bool> bool_list;
  OptionalList<int8_t> int8_list;
  OptionalList<int16_t> int16_list;
  OptionalList<int32_t> int32_list;
  OptionalList<int64_t> int64_list;
  OptionalList<uint8_t> u_int8_list;
  OptionalList<uint16_t> u_int16_list;
  OptionalList<uint32_t> u_int32_list;
  OptionalList<uint64_t> u_int64_list;
  OptionalList<float> float32_list;
  OptionalList<double> float64_list;
  OptionalList<Text> text_list;
  OptionalList<Data> data_list;
  OptionalList<AllTypesC> struct_list;
  OptionalList<Enum> enum_list;
};

extern "C" {

void __JUST_FOR_BINDGEN__(AllTypesC);

}  // extern "C"
