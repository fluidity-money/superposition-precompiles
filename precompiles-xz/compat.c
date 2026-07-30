#include <stddef.h>
#include <stdint.h>

int xz_memeq(const void *left_ptr, const void *right_ptr, size_t size)
{
    const uint8_t *left = left_ptr;
    const uint8_t *right = right_ptr;
    size_t i;
    uint8_t difference = 0;

    for (i = 0; i < size; ++i)
        difference |= left[i] ^ right[i];

    return difference == 0;
}

void *xz_memmove(void *destination, const void *source, size_t size)
{
    uint8_t *dst = destination;
    const uint8_t *src = source;
    size_t i;

    if (dst < src) {
        for (i = 0; i < size; ++i)
            dst[i] = src[i];
    } else if (dst > src) {
        for (i = size; i != 0; --i)
            dst[i - 1] = src[i - 1];
    }

    return destination;
}
