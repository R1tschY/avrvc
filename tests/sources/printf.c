// TEST: "\x10\x20\xA0\xF0\0" >> USARTC0 >> "10:20:A0:F0:00:"

#include "uart_io.h"

int main(void) {
    int c;

    uart_init();
    stdio_init();

    do {
        c = uart_getchar();
        printf("%02x:", c);
    } while (c != 0);

    asm("break");
}
