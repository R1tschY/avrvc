#!/bin/bash -e

avr-gcc main.c -O0  -mmcu=atxmega128a4u -s -o main
avr-objcopy -I elf32-avr -O ihex main main.hex
avr-objcopy -I elf32-avr -O binary main main.bin
avr-objdump -d main | grep -P '^\s+\w+:' | cut -d$'\t' -f3-4 | sed 's/ *$//' > main.S
