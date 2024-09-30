#include "include/csr.h"
#include "include/io.h"

#define UART        0x10000000

/* THR:transmitter holding register */
#define UART_DAT    (UART+0x00) /* data register */
#define UART_IER    (UART+0x01) /* interrupt enable register */
#define UART_IIR    (UART+0x02) /* interrupt falg register (read only)*/
#define UART_FCR    (UART+0x02) /* FIFO control register (write only)*/
#define UART_LCR    (UART+0x03) /* line control register */
#define UART_MCR    (UART+0x04) /* MODEN control register */
#define UART_LSR    (UART+0x05) /* line status register */
#define UART_MSR    (UART+0x06) /* MODEN status register */
#define UART_DLL (UART+0x00) /* Prescaler register low 8 bits */
#define UART_DLM (UART+0x01) /* Prescaler register high 8 bits */

#define UART_LSR_ERROR 0x80 /* error */
#define UART_LSR_EMPTY 0x40 /* FIFO and shift register are empty */
#define UART_LSR_TFE 0x20 /* Transmit FIFO is empty */
#define UART_LSR_BI 0x10 /* Transmission interrupted */
#define UART_LSR_FE 0x08 /* Frame received without stop bit */
#define UART_LSR_PE 0x04 /* Parity error bit */
#define UART_LSR_OE 0x02 /* Data overflow */
#define UART_LSR_DR 0x01 /* FIFO has data */

void sbi_uart_send(char c)
{
	while((readb(UART_LSR) & UART_LSR_EMPTY) == 0)
		;

	writeb(c, UART_DAT);
}

void sbi_uart_send_string(char *str)
{
	int i;

	for (i = 0; str[i] != '\0'; i++)
		sbi_uart_send((char) str[i]);
}

void sbi_putchar(char c)
{
       if (c == '\n')
               sbi_uart_send('\r');
       sbi_uart_send(c);
}

static unsigned int uart16550_clock = 1843200;   // a common base clock
#define UART_DEFAULT_BAUD  115200

void sbi_uart_init()
{
	unsigned int divisor = uart16550_clock / (16 * UART_DEFAULT_BAUD);

	/* disable interrupt */
	writeb(0, UART_IER);

	/* Enable DLAB (set baud rate divisor)*/
	writeb(0x80, UART_LCR);
	writeb((unsigned char)divisor, UART_DLL);
	writeb((unsigned char)(divisor >> 8), UART_DLM);

	/*8 bits, no parity, one stop bit*/
	writeb(0x3, UART_LCR);

	/* enable FIFO，clear FIFO，set 14-byte threshold */
	writeb(0xc7, UART_FCR);

    return;
}
