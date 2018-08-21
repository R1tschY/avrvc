// TEST: "TEST-base64/decode\0" >> USARTC0 >> "VEVTVC1iYXNlNjQvZW5jb2RlCg=="

#include "uart_io.h"
#include "base64.h"

int main(void) {
    char c;
    char input[101];
    char result[138];
    unsigned input_len;

    uart_init();
    input_len = uart_gets('\0', input, 100);

    Base64encode(result, input, input_len);

    uart_puts(result);

    asm("break");
}
