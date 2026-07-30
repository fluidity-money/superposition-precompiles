#ifndef PRECOMPILES_XZ_DECOMPRESS_H
#define PRECOMPILES_XZ_DECOMPRESS_H

#include <stddef.h>
#include <stdint.h>

#include "xz.h"

/*
 * Decompress one complete .xz stream into the caller-provided buffer.
 * *output_len is the output capacity on entry and the byte count on return.
 */
enum xz_ret xz_decompress(const uint8_t *input, size_t input_len,
                          uint8_t *output, size_t *output_len);

#endif
