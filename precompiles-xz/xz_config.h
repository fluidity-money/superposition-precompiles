/* Freestanding XZ Embedded configuration for wasm32 Stylus. */
#ifndef XZ_CONFIG_H
#define XZ_CONFIG_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/* Standard xz uses CRC64 by default. CRC32 is always built by XZ Embedded. */
#ifndef XZ_USE_CRC64
#define XZ_USE_CRC64
#endif

#include "xz.h"

#define kmalloc(size, flags) malloc(size)
#define kfree(ptr) free(ptr)
#define vmalloc(size) malloc(size)
#define vfree(ptr) free(ptr)

int xz_memeq(const void *left, const void *right, size_t size);
void *xz_memmove(void *destination, const void *source, size_t size);

#define memeq(a, b, size) xz_memeq((a), (b), (size))
#define memzero(buf, size) __builtin_memset((buf), 0, (size))
#define memcpy(destination, source, size) \
    __builtin_memcpy((destination), (source), (size))
#define memset(destination, value, size) \
    __builtin_memset((destination), (value), (size))
#define memmove(destination, source, size) \
    xz_memmove((destination), (source), (size))

#ifndef min
#define min(x, y) ((x) < (y) ? (x) : (y))
#endif
#define min_t(type, x, y) min((x), (y))

#ifndef fallthrough
#define fallthrough __attribute__((__fallthrough__))
#endif

#ifndef __always_inline
#define __always_inline inline __attribute__((__always_inline__))
#endif

static inline uint32_t get_unaligned_le32(const uint8_t *buf)
{
    return (uint32_t)buf[0]
        | ((uint32_t)buf[1] << 8)
        | ((uint32_t)buf[2] << 16)
        | ((uint32_t)buf[3] << 24);
}

static inline uint32_t get_unaligned_be32(const uint8_t *buf)
{
    return ((uint32_t)buf[0] << 24)
        | ((uint32_t)buf[1] << 16)
        | ((uint32_t)buf[2] << 8)
        | (uint32_t)buf[3];
}

static inline void put_unaligned_le32(uint32_t value, uint8_t *buf)
{
    buf[0] = (uint8_t)value;
    buf[1] = (uint8_t)(value >> 8);
    buf[2] = (uint8_t)(value >> 16);
    buf[3] = (uint8_t)(value >> 24);
}

static inline void put_unaligned_be32(uint32_t value, uint8_t *buf)
{
    buf[0] = (uint8_t)(value >> 24);
    buf[1] = (uint8_t)(value >> 16);
    buf[2] = (uint8_t)(value >> 8);
    buf[3] = (uint8_t)value;
}

#define get_le32 get_unaligned_le32
#define get_be32 get_unaligned_be32

#endif
