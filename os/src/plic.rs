#[allow(dead_code)]

const MAX_CPUS: i32 = 1;
const MAX_PLIC_IRQS: i32 = 53;

const PLIC_BASE: usize =  0xc000000;

/* irq extern interrupt enable */
const SIE_SEIE: usize = 0x200;

use crate::csr_set;
use crate::csr_clear;
use crate::trap::PtRegs;
use crate::io::*;
use crate::println;
use crate::drivers::serial::uart16550::handle_uart_irq;

/* configure interrupts priority */
macro_rules! PLIC_PRIORITY {
    ($hwirq:ident) => {
        PLIC_BASE + (($hwirq * 4) as usize)
    };
}

/* configure interrupts pending bit, one bit -> one interrupt */
#[allow(unused_macros)]
macro_rules! PLIC_PENDING {
    ($hwirq:ident) => {
        PLIC_BASE + 0x1000 + ((($hwirq / 32) * 4) as usize)
    };
}

/* configure interrupts enabled */
macro_rules! PLIC_MENABLE {
    ($hart:ident) => {
        PLIC_BASE + 0x2000 + (($hart * 0x80) as usize)
    };
}

/*
 * configure interrupts mthreshold,
 * if 'priority' > 'mthreshold' then the interrupt will trigger
 */
macro_rules! PLIC_MTHRESHOLD {
    ($hart:ident) => {
        PLIC_BASE + 0x200004 + (($hart * 0x1000) as usize)
    };
}

/*
 * The PLIC interrupt controller supports multiple interrupt hardware contexts,
 * each hart may contain multiple interrupt contexts, and each interrupt context contains a set of interrupt-related control registers.
 * This macro maps from CPU to hart, and then maps to the M mode interrupt context.
 *
 * For the QEMU Virt platform, CPU0 maps to hart1, CPU1 maps to hart2, and so on.
 * Each hart has M mode and S mode interrupt contexts.
 * The CPU_TO_CONTEXT macro is used to map the M mode interrupt context corresponding to the hart of the CPU.
 */
macro_rules! CPU_TO_CONTEXT {
    ($cpu:ident) => {
        $cpu * 2 + 1
    };
}

macro_rules! PLIC_MCLAIM {
    ($context:ident) => {
        PLIC_BASE + 0x200004 + (($context * 0x1000) as usize)
    };
}

fn plic_set_prority(hwirq: i32, pro: u32) {
	let reg = PLIC_PRIORITY!(hwirq);

	writel(pro, reg);
}

pub fn plic_enable_irq(cpu: i32, hwirq: i32, enable: bool) {
    let hwirq_mask = 1 << (hwirq % 32);
    let context = CPU_TO_CONTEXT!(cpu);
	let reg = PLIC_MENABLE!(context) + (4 * (hwirq / 32) as usize);

	println!("reg: 0x{:x}, hwirq:{:?}, enabled: {}", reg, hwirq, enable);

	if enable {
		writel(readl(reg) | hwirq_mask, reg);
    } else {
		writel(readl(reg) & !hwirq_mask, reg);
    }
}

pub fn plic_init() -> i32 {
    for i in 0..MAX_CPUS {
        let context = CPU_TO_CONTEXT!(i) as usize;
        /* congigure m-mode all cpu hart interrupt threshold to 0 */
        writel(0, PLIC_MTHRESHOLD!(context));

        for hwirq in 1..=MAX_PLIC_IRQS {
            /* Disabled all external interrupts int the PLIC */
            plic_enable_irq(i, hwirq, false);

            /* Pre-set the priority of all interrupt numbers to 1 */
            plic_set_prority(hwirq, 1);
        }
    }

    csr_set!(sie, SIE_SEIE);
    0
}

#[allow(unused_variables)]
pub fn handler_plic_irq(regs: &PtRegs) {
    let mut hwirq: u32;
    /* only use CPU0 handle */
    let cpu: i32 = 0;
    let context = CPU_TO_CONTEXT!(cpu);
	let claim_reg = PLIC_MCLAIM!(context);

    csr_clear!(sie, SIE_SEIE);

    loop {
        hwirq = readl(claim_reg);
        if hwirq == 0 {
            break;
        }

        /* UART0_IRQ is 10 in qumu virt */
        if hwirq == 10 {
            handle_uart_irq();
        }

        writel(hwirq, claim_reg);
    }

    csr_set!(sie, SIE_SEIE);
}
