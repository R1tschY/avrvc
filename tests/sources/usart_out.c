#include <avr/io.h>

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

void uart_puts(char * str) {
    while (*str) {
        uart_putchar(*str++);
    }
}

int main(void) {
    uart_init();
    uart_puts("Hello World!\n");
    asm("break");
}
