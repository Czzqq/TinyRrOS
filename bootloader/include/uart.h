#ifndef UART_H_
#define UART_H_

void sbi_uart_init();
void sbi_putchar(char c);
void sbi_uart_send_string(char *str);

#endif
