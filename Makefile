.PHONY: bootloader os clean run

GDB_TOOL := gdb

O := $(PWD)/out
OS_TARGET := os
BOOTLOADER_TARGET := bootloader

all: dir bootloader os

# Target to create the directory if it doesn't exist
dir:
	@if [ ! -d "$(O)" ]; then \
		mkdir -p "$(O)"; \
		mkdir -p "$(O)/$(OS_TARGET)"; \
		mkdir -p "$(O)/$(BOOTLOADER_TARGET)"; \
		echo "Directory $(OUT) created."; \
	else \
		echo "Directory $(OUT) already exists."; \
	fi

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
QEMU_BIOS = -bios $(O)/bootloader/$(BOOTLOADER_TARGET)  -device loader,file=$(O)/os/$(OS_TARGET).bin,addr=0x80200000

run:
	qemu-system-riscv64 $(QEMU_FLAGS) $(QEMU_BIOS) -kernel $(O)/os/$(OS_TARGET)

debug:
	qemu-system-riscv64 $(QEMU_FLAGS) $(QEMU_BIOS) -kernel $(O)/os/$(OS_TARGET) -gdb tcp::1235 -S

gdb:
	$(GDB_TOOL) \
	-ex 'file ./out/os/os' \
	-ex 'set arch riscv:rv64' \
	-ex 'target remote localhost:1235'

