#include "include/csr.h"
#include "include/sbi_trap_regs.h"
#include "sbi_trap.h"
#include "include/uart.h"

extern void sbi_exception_vector(void);

void sbi_trap_init(void)
{
    /* set exception vection base address */
    write_csr(mtvec, sbi_exception_vector);

    /* close all int */
    write_csr(mie, 0);
}

static int sbi_ecall_handle(unsigned int id, struct sbi_trap_regs *regs)
{
    int ret = 0;

    switch (id) {
        case SBI_CONSOLE_PUTCHAR:
            sbi_putchar(regs->a0);
            ret = 0;
            break;
        default:
            break;
    }

    if (!ret)
        regs->mepc += 4;

    return ret;
}

void sbi_trap_handler(struct sbi_trap_regs *regs)
{
    unsigned long mcause = read_csr(mcause);
    unsigned long ecall_id = regs->a7;

    switch (mcause) {
        case CAUSE_SUPERVISOR_ECALL:
            sbi_ecall_handle(ecall_id, regs);
            break;
        default:
            break;
    }
}
