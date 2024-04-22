OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    INCLUDE "bootloader/sbi_base.ld"
}