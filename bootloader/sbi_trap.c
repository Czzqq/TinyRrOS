#include "include/csr.h"
#include "include/sbi_trap_regs.h"
#include "sbi_trap.h"
#include "include/uart.h"

extern void sbi_exception_vector(void);

void sbi_trap_init(void)
{
    /* set exception vection base address */
    write_csr(mtvec, sbi_exception_vector);

    /* disable all interrupt */
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

void delegate_traps(void)
{
	unsigned long interrupts;
	unsigned long exceptions;

	interrupts = MIP_SSIP | MIP_STIP | MIP_SEIP;
	exceptions = (1UL << CAUSE_MISALIGNED_FETCH) | (1UL << CAUSE_FETCH_PAGE_FAULT) |
                    (1UL << CAUSE_BREAKPOINT) | (1UL << CAUSE_LOAD_PAGE_FAULT) |
                    (1UL << CAUSE_STORE_PAGE_FAULT) | (1UL << CAUSE_USER_ECALL) |
                    (1UL << CAUSE_LOAD_ACCESS) | (1UL << CAUSE_STORE_ACCESS);

	 write_csr(mideleg, interrupts);
	 write_csr(medeleg, exceptions);
}
