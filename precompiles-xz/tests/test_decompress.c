#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "decompress.h"

static const char expected[] = "hello from xz-embedded\n";

int main(int argc, char **argv)
{
    unsigned char input[1024];
    unsigned char output[1024];
    size_t input_len;
    size_t output_len = sizeof(output);
    FILE *file;
    enum xz_ret ret;

    if (argc != 2)
        return 2;

    file = fopen(argv[1], "rb");
    if (file == NULL)
        return 3;

    input_len = fread(input, 1, sizeof(input), file);
    fclose(file);

    ret = xz_decompress(input, input_len, output, &output_len);
    if (ret != XZ_STREAM_END) {
        fprintf(stderr, "xz_decompress returned %d\n", ret);
        return 4;
    }

    if (output_len != sizeof(expected) - 1
            || memcmp(output, expected, sizeof(expected) - 1) != 0) {
        fputs("decompressed output did not match\n", stderr);
        return 5;
    }

    return 0;
}
