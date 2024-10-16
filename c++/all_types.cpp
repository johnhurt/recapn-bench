
#include "all_types.capnp.h"
#include <capnp/message.h>
#include <capnp/serialize-packed.h>
#include <kj/io.h>
#include <iostream>
#include <vector>
#include <string>
#include <stdexcept>
#include "recapn_bench_cpp.h"
#include "../fuzz-models/fuzz_models.h"

using namespace std;

TestEnum from_enum(Enum raw)
{
    switch (raw)
    {
    case Enum::Foo:
        return TestEnum::FOO;
    case Enum::Bar:
        return TestEnum::BAR;
    case Enum::Baz:
        return TestEnum::BAZ;
    case Enum::Qux:
        return TestEnum::QUX;
    case Enum::Quux:
        return TestEnum::QUUX;
    case Enum::Corge:
        return TestEnum::CORGE;
    case Enum::Grault:
        return TestEnum::GRAULT;
    case Enum::Garply:
        return TestEnum::GARPLY;
    }
}

void write_all_types_helper(
    TestAllTypes::Builder &target,
    AllTypesC *v,
    kj::OutputStream &output)
{
    target.setBoolField(v->bool_field);
    target.setInt8Field(v->int8_field);
    target.setInt16Field(v->int16_field);
    target.setInt32Field(v->int32_field);
    target.setInt64Field(v->int64_field);
    target.setUInt8Field(v->uint8_field);
    target.setUInt16Field(v->uint16_field);
    target.setUInt32Field(v->uint32_field);
    target.setUInt64Field(v->uint64_field);
    target.setFloat32Field(v->float32_field);
    target.setFloat64Field(v->float64_field);
    target.setEnumField(from_enum(v->enum_field));

    if (v->text_field.present)
    {
        target.setTextField(capnp::Text::Reader(
            (char *)v->text_field.value.start,
            (size_t)v->text_field.value.size));
    }

    if (v->data_field.present)
    {
        size_t size = (size_t)v->data_field.value.size;
        if (size > 0)
        {
            target.setDataField(capnp::Data::Reader(
                (u_int8_t *)v->data_field.value.start,
                size));
        }
        else
        {
            target.setDataField(kj::Array<u_int8_t>(0));
        }
    }
}

void write_all_types_to_buffer(AllTypesC *v, kj::OutputStream &output, bool packed)
{
    ::capnp::MallocMessageBuilder message;

    TestAllTypes::Builder root = message.initRoot<TestAllTypes>();

    write_all_types_helper(root, v, output);

    if (packed)
    {
        writePackedMessage(output, message);
    }
    else
    {
        writeMessage(output, message);
    }
}

Buffer serialize_all_types(AllTypesC *v, bool packed)
{

    Buffer result;
    result.kj_array_buffer = (void *)0;

    try
    {
        kj::VectorOutputStream *stream = new kj::VectorOutputStream(1024);

        write_all_types_to_buffer(v, *stream, packed);

        result.kj_array_buffer = (void *)stream;

        result.size = stream->getArray().asBytes().size();
    }
    catch (const std::exception &e)
    {
        // catch anything thrown within try block that derives from std::exception
        std::cerr << e.what();
    }

    return result;
}