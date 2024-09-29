#ifndef SBI_TRAP_H_
#define SBI_TRAP_H_

#define SBI_CONSOLE_PUTCHAR 0x1

/* ===== Trap/Exception Causes ===== */
#define CAUSE_SUPERVISOR_ECALL		0x9

void sbi_trap_init(void);

#endif
