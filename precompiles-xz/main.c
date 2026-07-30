#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#include "decompress.h"
#include "hostio.h"

#ifndef XZ_OUTPUT_CAPACITY
#define XZ_OUTPUT_CAPACITY (1024U * 1024U)
#endif

static uint8_t error_code;

__attribute__((export_name("mark_used")))
void mark_used(void)
{
    pay_for_memory_grow(0);
}

/*
 * msg.data is exactly one complete .xz stream.
 * Success: returns the decompressed bytes.
 * Revert: returns one byte containing enum xz_ret.
 */
__attribute__((export_name("user_entrypoint")))
int user_entrypoint(size_t args_len)
{
    uint8_t *input;
    uint8_t *output;
    size_t output_len = XZ_OUTPUT_CAPACITY;
    enum xz_ret ret;

    input = malloc(args_len == 0 ? 1 : args_len);
    output = malloc(XZ_OUTPUT_CAPACITY);
    if (input == NULL || output == NULL) {
        error_code = (uint8_t)XZ_MEM_ERROR;
        write_result(&error_code, 1);
        return 1;
    }

    /* read_args is the Stylus C SDK hostio that copies EVM msg.data. */
    read_args(input);

    ret = xz_decompress(input, args_len, output, &output_len);
    if (ret != XZ_STREAM_END) {
        error_code = (uint8_t)ret;
        write_result(&error_code, 1);
        return 1;
    }

    write_result(output, output_len);
    return 0;
}
