#!/usr/bin/env python3
# -*- coding: utf-8 -*-


def is_2word_instruction(instr: int):
  return (
      (instr >> 8) & 0b11111110 == 0b10010100 and instr & 0b1100 == 0b1100
  ) or (
      (instr >> 8) & 0b11111100 == 0b10010000 and instr & 0b1111 == 0b0000
  )


with open("all_instrs.bin", "wb") as fp:
    for instr in range(0, 0x10000):
        fp.write(instr.to_bytes(2, "little"))
        if is_2word_instruction(instr):
            fp.write(b"\x00\x00")

