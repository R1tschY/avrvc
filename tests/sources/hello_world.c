// TEST: "" >> USARTC0 >> "Hello World!\n"

#include "uart_io.h"

int main(void) {
    uart_init();

    uart_puts("Hello World!\n");

    asm("break");
}
