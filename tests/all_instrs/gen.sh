#!/bin/bash -e

./generate_instr_test.py
avr-objcopy -I binary -O elf32-avr all_instrs.bin all_instrs.elf
avr-objdump -D all_instrs.elf | grep -P '^\s+\w+:' | cut -d$'\t' -f3-4 | sed 's/ *$//' > main.S
