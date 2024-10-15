.PHONY: all build

TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/os
OS_PATH := $(PWD)/os
SYMBOLS_FILE := $(OS_PATH)/src/symbols.rs

all: build install

build:
	@echo "Build os..."
	@cargo build
	@rust-nm $(KERNEL_ELF) > $(O)/symbols.txt
	@cp $(O)/symbols.txt $(O)/first_symbols.txt
	@python3 $(OS_PATH)/scripts/symbol2rs.py $(O)/symbols.txt $(SYMBOLS_FILE)
	@cargo build --features with-symbol-table
	@rust-nm $(KERNEL_ELF) > $(O)/symbols.txt
	@cp $(O)/symbols.txt $(O)/second_symbols.txt
	@python3 $(OS_PATH)/scripts/symbol2rs.py $(O)/symbols.txt $(SYMBOLS_FILE)
	@cargo build --features with-symbol-table
	@rust-nm $(KERNEL_ELF) > $(O)/final_symbols.txt

clean:
	@rm $(SYMBOLS_FILE)
	@cargo clean

install: build
	@echo "copy os to out..."
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_ELF).bin
	@cp $(KERNEL_ELF) $(O)/
	@cp $(KERNEL_ELF).bin $(O)/
