#include "decompress.h"

enum xz_ret xz_decompress(const uint8_t *input, size_t input_len,
                          uint8_t *output, size_t *output_len)
{
    struct xz_buf buffer = {
        .in = input,
        .in_pos = 0,
        .in_size = input_len,
        .out = output,
        .out_pos = 0,
        .out_size = *output_len,
    };
    struct xz_dec *decoder;
    enum xz_ret ret;

    xz_crc32_init();
#ifdef XZ_USE_CRC64
    xz_crc64_init();
#endif

    /* XZ_SINGLE uses output as the LZMA2 history buffer. */
    decoder = xz_dec_init(XZ_SINGLE, 0);
    if (decoder == NULL) {
        *output_len = 0;
        return XZ_MEM_ERROR;
    }

    ret = xz_dec_run(decoder, &buffer);
    xz_dec_end(decoder);
    *output_len = buffer.out_pos;
    return ret;
}
