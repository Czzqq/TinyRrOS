#include "sbi_timer.h"
#include "include/io.h"
#include "include/clint.h"
#include "sbi_trap.h"
#include "include/csr.h"
#include "include/clint.h"

void sbi_timer_process(void)
{
    /*
     * disabled timer interrupts which in M-mode,
     * configure timer pending interrupts which in S-mode
     */
	csr_clear(mie, MIP_MTIP);
	csr_set(mip, MIP_STIP);
}

void clint_timer_event_start(unsigned long next_event)
{
	/* Program CLINT Time Compare */
    writeq(next_event, VIRT_CLINT_TIMER_CMP);

    /*
     * clear timer pending interrupts which in s-mode,
     * enable timer interrupts which in M-mode
     */
	csr_clear(mip, MIP_STIP);
	csr_set(mie, MIP_MTIP);
}
