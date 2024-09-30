#include "include/csr.h"
#include "sbi_lib.h"
#include "sbi_trap.h"
#include "include/uart.h"

#define FW_JUMP_ADDR 0x80200000

int sbi_set_pmp(int reg_idx, unsigned long start, unsigned long size, unsigned long prot)
{
	int order;
	int pmpcfg_csr, pmpcfg_shift, pmpaddr_csr;
	unsigned long cfgmask, pmpcfg;
	unsigned long addrmask, pmpaddr;

	if (reg_idx > MAX_CSR_PMP)
		return -1;

	order = log2roundup(size);
	if (order < PMP_SHIFT)
		return -1;

    //printk("%s: start: 0x%lx order %d prot 0x%lx\n", __func__, start, order, prot);

	pmpaddr = start >> PMP_SHIFT;

	/* For RV64, the corresponding cfg registers are pmpcfg0, pmpcfg2, pmpcfg4... */
	pmpcfg_csr   = (CSR_PMPCFG0 + (reg_idx >> 2)) & ~1;
	pmpcfg_shift = (reg_idx & 7) << 3;

	pmpaddr_csr = CSR_PMPADDR0 + reg_idx;

	/* Configure the A field in the cfg; NA4 indicates a region of only 4 bytes. */
	prot &= ~PMP_A;
	prot |= (order == PMP_SHIFT) ? PMP_A_NA4 : PMP_A_NAPOT;

	/* Configure prot of cfg */
	cfgmask = ~(0xffUL << pmpcfg_shift);
	pmpcfg	= (read_csr_num(pmpcfg_csr) & cfgmask);
	pmpcfg |= ((prot << pmpcfg_shift) & ~cfgmask);

	/*
	 * Configure PMP address
	 * if oder == 2，A uses PMP_A_NA4, and pmpaddr directly uses start>>2
	 * if oder > 2，A uses PMP_A_NAPOT，and needs to reconfigure pmpaddr
	 */
	if (order > PMP_SHIFT)
	{
		if (order == RISCV_XLEN) {
			pmpaddr = -1UL;
		} else {
			/*
			 * if the value of pmpaddr is y...y01...1，and the number of consecutive 1s is n,
             * then the address space controlled by this PMP entry starts from y...y00...0 and spans 2^{n+3} bytes.
             * Refer to the RISC-V manual.
			 */
			addrmask = (1UL << (order - PMP_SHIFT)) - 1;
			pmpaddr	 &= ~addrmask;
			pmpaddr |= (addrmask >> 1);
		}
	}

    //printk("%s: pmpaddr: 0x%lx  pmpcfg 0x%lx, cfs_csr 0x%x addr_csr 0x%x\n",
	//		__func__, pmpaddr, pmpcfg, pmpcfg_csr, pmpaddr_csr);

	write_csr_num(pmpaddr_csr, pmpaddr);
	write_csr_num(pmpcfg_csr, pmpcfg);

	return 0;
}

/*
 *  switch to s-mode
 */
void sbi_start(void)
{
    unsigned long val;

    sbi_uart_init();

    /* Set the pmp */
    sbi_set_pmp(0, 0, -1UL, PMP_RWX);
    sbi_set_pmp(1, 0x80000000, 0x40000, PMP_RWX);

	sbi_trap_init();

    val = read_csr(mstatus);
    val = INSERT_FIELD(val, MSTATUS_MPP, PRV_S);
    val = INSERT_FIELD(val, MSTATUS_MPIE, 0);
    write_csr(mstatus, val);

    /* set m-mode mepc, for mret jump */
    write_csr(mepc, FW_JUMP_ADDR);
    /* set s-mode exception vector address */
    write_csr(stvec, FW_JUMP_ADDR);
    /* disable s-mode interrupts */
    write_csr(sie, 0);
    /* disable s-mode page table translation */
    write_csr(satp, 0);

    /* jump to s-mode */
    asm volatile("mret");
}
