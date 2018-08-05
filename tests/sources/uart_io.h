#include <stdio.h>
#include <avr/io.h>

// UART

void uart_init(void) {
    USARTC0_BAUDCTRLB = 0;
    USARTC0_BAUDCTRLA = 0x84;
    USARTC0.CTRLA = USART_RXCINTLVL_HI_gc;
    USARTC0.CTRLB = USART_TXEN_bm | USART_RXEN_bm;
    USARTC0.CTRLC = USART_CHSIZE_8BIT_gc;
}

void uart_putchar(char c) {
    while (!(USARTC0_STATUS & USART_DREIF_bm)) {};
    USARTC0_DATA = c;
}

void uart_puts(char* str) {
    while (*str) {
        uart_putchar(*str++);
    }
}

char uart_getchar() {
    while (!(USARTC0_STATUS & USART_RXCIF_bm) );
    return USARTC0_DATA;
}

// STDIO

static int stdout_putchar(char c, FILE* stream) {
    uart_putchar(c);
    return 0;
}

int stdin_getchar(FILE* stream)
{
    return uart_getchar();
}

static FILE stdio =
    FDEV_SETUP_STREAM(stdout_putchar, stdin_getchar, _FDEV_SETUP_RW);

void stdio_init(void) {
    stdout = stdin = &stdio;
}