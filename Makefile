SRC := hsdt-unpack.zig hsdt.zig

.PHONY: all clean

all: hsdt-unpack hsdt-unpack.wasm

clean:
	rm -f hsdt-unpack hsdt-unpack.o hsdt-unpack.wasm hsdt-unpack.wasm.o

hsdt-unpack: $(SRC)
	zig build-exe -O ReleaseFast $<

hsdt-unpack.wasm: $(SRC)
	zig build-exe -O ReleaseFast -target wasm32-wasi $<
