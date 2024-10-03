#include "include/csr.h"
#include "include/sbi_trap_regs.h"
#include "sbi_trap.h"
#include "sbi_timer.h"
#include "sbi_error.h"
#include "include/uart.h"

extern void sbi_exception_vector(void);


void sbi_panic()
{
	sbi_uart_send_string("sbi panic, goto loop\n");
	while(1)
		;
}

static void sbi_trap_error(struct sbi_trap_regs *regs, const char *msg, int rc)
{
	sbi_panic();
}

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
        case SBI_SET_TIMER:
            clint_timer_event_start(regs->a0);
            ret = 0;
            break;
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
    int rc = SBI_ENOTSUPP;
    const char *msg = "trap handler failed";

    if (mcause & MCAUSE_IRQ) {
        mcause &= ~MCAUSE_IRQ;
        switch (mcause) {
            case IRQ_M_TIMER:
                sbi_timer_process();
            break;
            default:
                msg = "unhandled external interrupt";
                goto trap_error;
        }
    }

    switch (mcause) {
        case CAUSE_SUPERVISOR_ECALL:
            sbi_ecall_handle(ecall_id, regs);
            break;
        default:
            break;
    }
trap_error:
    if (rc) {
        sbi_trap_error(regs, msg, rc);
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
