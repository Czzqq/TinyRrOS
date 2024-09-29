//! 16550 serial driver for qemu riscv virt
#![allow(dead_code)]

const UART_BASE: usize = 0x10000000;

const UART_DAT: usize = UART_BASE + 0x00;
const UART_IER: usize = UART_BASE + 0x01;
const UART_IIR: usize = UART_BASE + 0x02;
const UART_FCR: usize = UART_BASE + 0x02;
const UART_LCR: usize = UART_BASE + 0x03;
const UART_MCR: usize = UART_BASE + 0x04;
const UART_LSR: usize = UART_BASE + 0x05;
const UART_MSR: usize = UART_BASE + 0x06;

const UART_DLL: usize = UART_BASE + 0x00;
const UART_DLM: usize = UART_BASE + 0x01;

const UART_LSR_ERROR: usize = 0x80;
const UART_LSR_EMPTY: usize = 0x40;
const UART_LSR_TFE: usize = 0x20;
const UART_LSR_BI: usize = 0x10;
const UART_LSR_FE: usize = 0x08;
const UART_LSR_PE: usize = 0x04;
const UART_LSR_OE: usize = 0x02;
const UART_LSR_DE: usize = 0x01;

static UART16550_CLOCK: u32 = 1843200;
const UART_DEFAULT_BAUD: u32 = 115200;

use crate::io;

fn uart_send(c: char) {
    while (io::readb(UART_LSR) & UART_LSR_EMPTY as u8) == 0 {
        // busy wait until the uart is ready to send
    }

    io::writeb(c as u8, UART_DAT);
}

pub fn uart_send_string(str: &str) {
    for c in str.chars() {
        uart_send(c);
    }
}

pub fn uart_init() {
    let divisor: u32 = UART16550_CLOCK / (16 * UART_DEFAULT_BAUD);

    /* disabled interrupt */
    io::writeb(0, UART_IER);

    /* enable DLAB (set baud rate divisor) */
    io::writeb(0x80, UART_LCR);
    io::writeb(divisor as u8, UART_DLL);
    io::writeb((divisor >> 8) as u8, UART_DLM);

    /* 8bits, no parity, one stop bit */
    io::writeb(0x3, UART_LCR);

    /* enable FIFO, empty FIFO, set 14 bytes threshold */
    io::writeb(0xc7, UART_FCR);
}