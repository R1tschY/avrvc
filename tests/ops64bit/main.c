#include <stdint.h>

void trap() {
  asm("break");
}


uint64_t c1 = 0x7a69;
uint64_t c2 = 0xa59068FF;

uint64_t rsa(int64_t code, int64_t c1, int64_t c2) {
  int64_t v33 = c1;
  int64_t v1 = 1;
  int64_t v9 = code;
 
  for (v33 = c1; v33 != 0; v33 >>= 1)
  {
      if (v33 & 1)
      {
        v1 = (v1 * v9) % c2;
      }
      v9 = (v9 * v9) % c2;
  }
 
  return v1 % c2; 

}


int main() {
  uint64_t _ = rsa(0x1234, c1, c2);
  trap();
	return _;
}
