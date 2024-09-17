#define FUNC_SIGS(struct_name)                 \
    int read_##struct_name(void *, int, bool); \
    int write_##struct_name(void *, int, bool);

FUNC_SIGS(addressbook)
FUNC_SIGS(all_types)