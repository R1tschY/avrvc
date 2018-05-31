#include <stdint.h>

int main() {
  uint64_t sum = 0;
	for (uint64_t i = 0; i < 2000000; i++) {
	    sum += i;
	}
	
	asm("break");	
	return sum;
}
