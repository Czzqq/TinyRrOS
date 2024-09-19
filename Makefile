.PHONY: bootloader os clean run

O := $(PWD)/out
OS_TARGET := os
BOOTLOADER_TARGET := bootloader

all: bootloader os

bootloader:
	@$(MAKE) O=$(O)/bootloader -C bootloader

os:
	@$(MAKE) O=$(O)/os -C os

clean:
	@$(MAKE) -C os clean
	@$(MAKE) -C bootloader clean
	@rm -rf $(O)/os/*
	@rm -rf $(O)/bootloader/*

QEMU_FLAGS  += -nographic -machine virt -m 128M
QEMU_BIOS = -bios $(O)/bootloader/$(BOOTLOADER_TARGET)  -device loader,file=$(O)/os/$(OS_TARGET),addr=0x80200000

run:
	qemu-system-riscv64 $(QEMU_FLAGS) $(QEMU_BIOS) -kernel $(O)/os/$(OS_TARGET)

debug:
	qemu-system-riscv64 $(QEMU_FLAGS) $(QEMU_BIOS) -kernel $(O)/os/$(OS_TARGET) -gdb tcp::1235 -S
