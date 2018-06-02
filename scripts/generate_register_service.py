#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import logging
import subprocess
import sys
from pathlib import Path
import pystache

from elftools.elf.elffile import ELFFile

MCUS = [
    "atmega8", "atmega16",

    "atxmega16a4u", "atxmega32a4u", "atxmega64a4u", "atxmega128a4u"
]

IOREGS = [
    "SREG", "SPH", "SPL", "RAMPD", "RAMPX", "RAMPY", "RAMPZ",

    "PORTA", "PORTB", "PORTC", "PORTD",
    "PINA", "PINB", "PINC", "PIND",
    "DDRA", "DDRB", "DDRC", "DDRD",
]

REQUIRED_IOREGS = [
    "SREG", "SPH", "SPL"
]

CONSTANTS = [
    "RAMSTART", "RAMEND",
    "MAPPED_EEPROM_START", "MAPPED_EEPROM_END",
    "FLASHEND", "SPM_PAGESIZE",
    "IO_SIZE",

    "__AVR_ARCH__", "__AVR_XMEGA__", "__AVR_MEGA__",
    "__AVR_2_BYTE_PC__", "__AVR_3_BYTE_PC__"
]

REQUIRED_CONSTANTS = [
    "FLASHEND", "RAMSTART", "RAMEND", "__AVR_ARCH__"
]

DECTECTOR_TEMPLATE = """
#include <avr/io.h>

typedef volatile void* io_ptr_t;

{{#ioregs}}
#ifdef {{reg}}
io_ptr_t ioreg_{{reg}} = &{{reg}};
#endif

{{/ioregs}}

{{#constants}}
#ifdef {{constant}}
uint32_t constant_{{constant}} = {{constant}};
#endif

{{/constants}}

int main(void) { return 0; }
"""

RUST_TEMPLATE = """
// GENERATED - DO NOT EDIT!

use std::collections::HashMap;

pub type IoRegAddrs = HashMap<&'static str, usize>;

pub struct McuIoRegistersService {
    mcus: HashMap<&'static str, IoRegAddrs>
}

impl McuIoRegistersService {
    pub fn new() -> McuIoRegistersService {
        let mut service = McuIoRegistersService { mcus: HashMap::new() };

        {{#mcus}}
        let mut mcu_{{mcu}}: IoRegAddrs = HashMap::new();
        {{#regs}}
        mcu_{{mcu}}.insert("{{name}}", {{value}});
        {{/regs}}
        {{#constants}}
        mcu_{{mcu}}.insert("#{{name}}", {{value}});
        {{/constants}}
        service.mcus.insert("{{mcu}}", mcu_{{mcu}});
        
        {{/mcus}}

        service
    }

    pub fn get_mcu_registers(&self, mcu: &str) -> Option<&IoRegAddrs> {
        self.mcus.get(mcu)
    }
}
"""


logger = logging.getLogger("avrvc.mcu_generator")
renderer = pystache.Renderer(missing_tags='ignore', escape=lambda u: u)


def generate_ioreg_detector() -> str:
    return renderer.render(DECTECTOR_TEMPLATE, {
        "ioregs": [
            dict(reg=ioreg) for ioreg in IOREGS
        ], "constants": [
            dict(constant=constant) for constant in CONSTANTS
        ]
    })


def compile_file(filename: Path, destination: Path, mcu: str) -> None:
    subprocess.check_call([
        "avr-gcc", filename, "-o", destination, f"-mmcu={mcu}"
    ])


def generate_rust_file(mcus) -> str:
    return renderer.render(RUST_TEMPLATE, {
        "mcus": mcus
    })


def extract_value(data, data_offset, symbol):
    offset = symbol['st_value'] - data_offset
    d = data[offset:offset + symbol['st_size']]
    return int.from_bytes(d, "little")


def extract_group(name, required, data, data_offset, symbols_section):
    prefix = name + "_"
    values = [
        dict(
            name=symbol.name[len(prefix):],
            value=hex(extract_value(data, data_offset, symbol))
        )
        for symbol in symbols_section.iter_symbols()
        if symbol.name.startswith(prefix)
    ]
    
    missing_values = set(required) - set(c["name"] for c in values)
    if len(missing_values) > 1:
        raise RuntimeError(
            f"required {name}s {', '.join(missing_values)} are missing")
    elif missing_values:
        raise RuntimeError(
            f"required {name} {', '.join(missing_values)} is missing")
    
    return values


def extract_ioregs(elfFile: Path):
    with open(elfFile, 'rb') as f:
        elffile = ELFFile(f)

        data_section = elffile.get_section_by_name(".data")
        symbols_section = elffile.get_section_by_name(".symtab")

        if data_section is None:
            logger.critical(".data section missing")
            sys.exit(1)

        if symbols_section is None:
            logger.critical(".symtab section missing")
            sys.exit(1)

        data = data_section.data()
        data_offset = data_section['sh_addr']

        return {
            "regs": extract_group(
                "ioreg", REQUIRED_IOREGS,
                data, data_offset, symbols_section),

            "constants": extract_group(
                "constant", REQUIRED_CONSTANTS,
                data, data_offset, symbols_section)
        }


def main():
    mcu_ioregs = []

    build_dir = (Path(__file__) / ".." / "build").resolve(strict=False)
    build_dir.mkdir(exist_ok=True)

    print(f"generate detector ...")
    ioreg_detector_path = build_dir / "detector.c"
    with open(ioreg_detector_path, "w") as fp:
        fp.write(generate_ioreg_detector())

    for mcu in MCUS:
        print(f"generate for {mcu} ...")
        detector_elf_path = build_dir / f"{mcu}.elf"
        compile_file(ioreg_detector_path, detector_elf_path, mcu)
        mcu_ioregs.append(extract_ioregs(detector_elf_path))
        mcu_ioregs[-1]["mcu"] = mcu

    print(f"generate rust file ...")
    rust_service_path = \
        build_dir / ".." / ".." / "src" / "models" / "register_service.rs"
    with open(rust_service_path, "w") as fp:
        fp.write(generate_rust_file(mcu_ioregs))


if __name__ == '__main__':
    main()
