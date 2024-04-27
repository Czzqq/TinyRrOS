## 介绍
这是一个极其简单的 SBi 以及 Bootloader，使用到的代码都是来自 《RISC-V 体系结构编程与实现》 当中，并且为了保证更简单的 Makefile 仅仅有编译相关的内容，运行 qemu 则直接采用命令：
```
##############
#  run qemu
##############
ifeq ($(board), qemu)
QEMU_FLAGS  += -nographic -machine virt -m 128M 
QEMU_BIOS = -bios mysbi.bin  -device loader,file=benos.bin,addr=0x80200000 
run:
	qemu-system-riscv64 $(QEMU_FLAGS) $(QEMU_BIOS) -kernel benos.elf
debug:
	qemu-system-riscv64 $(QEMU_FLAGS) $(QEMU_BIOS) -kernel benos.elf -S -s
payload:
	qemu-system-riscv64 $(QEMU_FLAGS) -bios none -device loader,file=benos_payload.bin,addr=0x80000000

else ifeq ($(board), nemu)
run:
	riscv64-nemu-interpreter -b benos_payload.bin
debug:
	riscv64-nemu-interpreter benos_payload.bin
endif
```

## 参考

- [‘virt’ Generic Virtual Platform (virt) — QEMU documentation](https://www.qemu.org/docs/master/system/riscv/virt.html)
- 《RISC-V 体系结构编程与实现》