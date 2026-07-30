XZ_EMBEDDED ?= vendor/xz-embedded
STYLUS_SDK ?= vendor/stylus-sdk-c

WASM_CC ?= clang
HOST_CC ?= cc
WASM_LD ?= wasm-ld
WASM_STRIP ?= wasm-strip
STACK_SIZE ?= 16384
XZ_OUTPUT_CAPACITY ?= 1048576

XZ_LIB := $(XZ_EMBEDDED)/linux/lib/xz
XZ_INCLUDE := $(XZ_EMBEDDED)/linux/include/linux
STYLUS_INCLUDE := $(STYLUS_SDK)/include

XZ_SOURCES := \
	$(XZ_LIB)/xz_crc32.c \
	$(XZ_LIB)/xz_crc64.c \
	$(XZ_LIB)/xz_dec_stream.c \
	$(XZ_LIB)/xz_dec_lzma2.c
SOURCES := main.c decompress.c compat.c $(STYLUS_SDK)/src/stdlib.c $(XZ_SOURCES)
OBJECTS := $(patsubst %.c,build/%.o,$(notdir $(SOURCES)))

CPPFLAGS := -DXZ_USE_CRC64 -I. -I$(STYLUS_INCLUDE) -I$(XZ_INCLUDE) -I$(XZ_LIB)
CFLAGS := --target=wasm32 -std=gnu11 -Oz --no-standard-libraries \
	-mbulk-memory -Wall -Wextra -DXZ_OUTPUT_CAPACITY=$(XZ_OUTPUT_CAPACITY)
LDFLAGS := -O2 --no-entry --stack-first -z stack-size=$(STACK_SIZE) -Bstatic

.PHONY: all deps clean test
all: deps
	$(MAKE) precompiles-xz.wasm

deps:
	@mkdir -p "$(dir $(XZ_EMBEDDED))" "$(dir $(STYLUS_SDK))"
	@test -d "$(XZ_EMBEDDED)" || git clone --depth 1 https://github.com/tukaani-project/xz-embedded.git "$(XZ_EMBEDDED)"
	@test -d "$(STYLUS_SDK)" || git clone --depth 1 https://github.com/OffchainLabs/stylus-sdk-c.git "$(STYLUS_SDK)"

build:
	mkdir -p $@

build/%.o: %.c | build
	$(WASM_CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

build/stdlib.o: $(STYLUS_SDK)/src/stdlib.c | build
	$(WASM_CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

build/xz_%.o: $(XZ_LIB)/xz_%.c | build
	$(WASM_CC) $(CPPFLAGS) $(CFLAGS) -c $< -o $@

precompiles-xz.unstripped.wasm: $(OBJECTS)
	$(WASM_LD) $(LDFLAGS) $^ -o $@

precompiles-xz.wasm: precompiles-xz.unstripped.wasm
	$(WASM_STRIP) -o $@ $<

test:
	@printf 'hello from xz-embedded\n' > build-test-input
	@xz --check=crc64 --stdout build-test-input > build-test-input.xz
	$(HOST_CC) -std=gnu11 -O2 -Wall -Wextra \
		-DXZ_USE_CRC64 -I. -I$(XZ_INCLUDE) -I$(XZ_LIB) \
		tests/test_decompress.c decompress.c compat.c $(XZ_SOURCES) \
		-o build-test
	./build-test build-test-input.xz
	@rm -f build-test build-test-input build-test-input.xz

clean:
	rm -rf build build-test build-test-input build-test-input.xz \
		precompiles-xz.unstripped.wasm precompiles-xz.wasm
