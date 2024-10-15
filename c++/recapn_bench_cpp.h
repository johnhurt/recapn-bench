
#include "../fuzz-models/fuzz_models.h"

#define FUNC_SIGS(struct_name)                 \
    int read_##struct_name(void *, int, bool); \
    int write_##struct_name(void *, int, bool);

FUNC_SIGS(addressbook)
FUNC_SIGS(all_types)

struct Buffer
{
    void *kj_array_buffer;
    uint64_t size;
};

uint8_t *get_raw_buffer(Buffer *);
void drop_buffer(Buffer *);
uint64_t get_size(Buffer *);

Buffer serialize_all_types(AllTypesC *, bool);
