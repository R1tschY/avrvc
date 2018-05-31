#!/bin/bash -e

avr-gcc main.c -O0 -mmcu=atxmega128a4u -g -o main.elf
avr-objcopy -I elf32-avr -O ihex main.elf main.hex
avr-objcopy -I elf32-avr -O binary main.elf main.bin
avr-objdump -d main.elf | grep -P '^\s+\w+:' | cut -d$'\t' -f3-4 | sed 's/ *$//' > main.S
