# XZ Embedded Stylus example

ChatGPT created contract using C and embedded-xz. The contract treats the complete EVM
calldata (`msg.data`) as one `.xz` stream. It returns the decompressed bytes directly. On
an XZ decoding error it reverts with one byte containing the numeric `enum xz_ret` value
from XZ Embedded.

The output is bounded to 1 MiB by default. Change it at build time with, for example,
`make XZ_OUTPUT_CAPACITY=4194304`. A stream exceeding the bound reverts with
`XZ_BUF_ERROR`.

Build:

    make deps
    make

Or point at existing checkouts:

    make XZ_EMBEDDED=/path/to/xz-embedded STYLUS_SDK=/path/to/stylus-sdk-c

Run the native decoder test:

    make deps
    make test

`read_args()` is the Stylus C SDK hostio that copies EVM `msg.data`; the
entrypoint receives its length as `args_len`.
