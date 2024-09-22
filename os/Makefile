.PHONY: all build

TARGET := riscv64gc-unknown-none-elf
MODE := debug
KERNEL_ELF := target/$(TARGET)/$(MODE)/os

all: build install

build:
	@echo "Build os..."
	@cargo build

clean:
	@cargo clean

install: build
	@echo "copy os to out..."
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_ELF).bin
	@cp $(KERNEL_ELF) $(O)/
	@cp $(KERNEL_ELF).bin $(O)/
