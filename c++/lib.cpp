
#include "all_types.capnp.h"
#include <capnp/message.h>
#include <capnp/serialize-packed.h>
#include <kj/io.h>
#include <iostream>
#include <vector>
#include <string>
#include "recapn_bench_cpp.h"
#include "../fuzz-models/fuzz_models.h"

using namespace std;

int hello(void *buf, int len)
{
    vector<string> msg{"Hello", "FFI", "World", "from", "VS Code", "and the C++ extension!"};

    for (const string &word : msg)
    {
        cout << word << " ";
    }
    cout << endl;

    return 0;
}

uint8_t *get_raw_buffer(Buffer *buf)
{
    kj::VectorOutputStream *vos = (kj::VectorOutputStream *)buf->kj_array_buffer;

    return (uint8_t *)vos->getArray().asBytes().begin();
}

uint64_t get_size(Buffer *buf)
{
    kj::VectorOutputStream *vos = (kj::VectorOutputStream *)buf->kj_array_buffer;

    return (uint64_t)vos->getArray().asBytes().size();
}

void drop_buffer(Buffer *buf)
{
    kj::VectorOutputStream *vos = (kj::VectorOutputStream *)buf->kj_array_buffer;
    delete vos;
}
