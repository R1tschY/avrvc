#!/bin/bash -e

avr-gcc main.c -Os  -mmcu=atxmega128a4u -g -o main.dbg.elf
avr-objcopy -I elf32-avr -O ihex main.dbg.elf main.hex
avr-objcopy -I elf32-avr -O binary main.dbg.elf main.bin
avr-objcopy -I binary -O elf32-avr main.bin main.elf
avr-objdump -D main.elf | grep -P '^\s+\w+:' | cut -d$'\t' -f3-4 | sed 's/ *$//' > main.S
