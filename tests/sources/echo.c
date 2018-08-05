// TEST: "ECHO ECHO\nECHO\0" >> USARTC0 >> "ECHO ECHO\nECHO\0"

#include "uart_io.h"

int main(void) {
    char c;

    uart_init();

    do {
        c = uart_getchar();
        uart_putchar(c);
    } while (c != 0);

    asm("break");
}
