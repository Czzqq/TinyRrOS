.PHONY: all build

TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
OS_PATH := $(PWD)/os
SYMBOLS_FILE := $(OS_PATH)/src/symbols.c

all: build install

build:
	@echo "Build os..."
	@cargo build
	@rust-nm $(KERNEL_ELF) > $(O)/symbols.txt
	@python3 $(OS_PATH)/scripts/symbol2c.py $(O)/symbols.txt $(SYMBOLS_FILE)
	@riscv64-linux-gnu-gcc -E $(OS_PATH)/src/symbols.c -o $(O)/symbols.i
	@riscv64-linux-gnu-gcc -S $(O)/symbols.i -o $(O)/symbols.s
	@riscv64-linux-gnu-as $(O)/symbols.s -o $(O)/symbols.o
	@cargo build --features with-symbol-table

clean:
	@rm $(SYMBOLS_FILE)
	@cargo clean

install: build
	@echo "copy os to out..."
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_ELF).bin
	@cp $(KERNEL_ELF) $(O)/
	@cp $(KERNEL_ELF).bin $(O)/
