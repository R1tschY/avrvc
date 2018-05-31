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
    "SREG", "SPH", "SPL",

    "PORTA", "PORTB", "PORTC", "PORTD",
    "PINA", "PINB", "PINC", "PIND",
    "DDRA", "DDRB", "DDRC", "DDRD",
]

DECTECTOR_TEMPLATE = """
#include <avr/io.h>

typedef volatile void* io_ptr_t;

{{#detectors}}
#ifdef {{reg}}
io_ptr_t ioreg_{{reg}} = &{{reg}};
#endif

{{/detectors}}

int main(void) { return 0; }
"""

RUST_TEMPLATE = """
// GENERATED - DO NOT EDIT!

use std::collections::HashMap;

pub type IoRegAddrs = HashMap<&'static str, usize>;

pub struct McuIoRegistersService {
    mcus: HashMap<&'static str, HashMap<&'static str, usize>>
}

impl McuIoRegistersService {
    pub fn new() -> McuIoRegistersService {
        let mut service = McuIoRegistersService { mcus: HashMap::new() };

        {{#mcus}}
        let mut mcu_{{mcu}}: HashMap<&'static str, usize> = HashMap::new();
        {{#regs}}
        mcu_{{mcu}}.insert("{{reg}}", {{addr}});
        {{/regs}}
        service.mcus.insert("{{mcu}}", mcu_{{mcu}});
        
        {{/mcus}}

        service
    }

    pub fn get_mcu_registers(&self, mcu: &str) -> Option<&HashMap<&'static str, usize>> {
        self.mcus.get(mcu)
    }
}
"""


logger = logging.getLogger("avrvc.mcu_generator")
renderer = pystache.Renderer(missing_tags='ignore', escape=lambda u: u)


def generate_ioreg_detector() -> str:
    return renderer.render(DECTECTOR_TEMPLATE, {
        "detectors": [
            dict(reg=ioreg) for ioreg in IOREGS
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

        ioreg_symbols = [
            symbol
            for symbol in symbols_section.iter_symbols()
            if symbol.name.startswith("ioreg_")
        ]

        regs = []
        for ioreg_symbol in ioreg_symbols:
            ioreg_name = ioreg_symbol.name[len("ioreg_"):]
            offset = ioreg_symbol['st_value'] - data_offset
            d = data[offset:offset + ioreg_symbol['st_size']]
            regs.append(dict(
                reg=ioreg_name,
                addr=hex(int.from_bytes(d, "little"))
            ))

        return {"regs": regs}


def main():
    mcu_ioregs = []

    build_dir = (Path(__file__) / ".." / "build").resolve(strict=False)
    build_dir.mkdir(exist_ok=True)

    ioreg_detector_path = build_dir / "detector.c"
    with open(ioreg_detector_path, "w") as fp:
        fp.write(generate_ioreg_detector())

    for mcu in MCUS:
        detector_elf_path = build_dir / f"{mcu}.elf"
        compile_file(ioreg_detector_path, detector_elf_path, mcu)
        mcu_ioregs.append(extract_ioregs(detector_elf_path))
        mcu_ioregs[-1]["mcu"] = mcu

    rust_service_path = \
        build_dir / ".." / ".." / "src" / "models" / "register_service.rs"
    with open(rust_service_path, "w") as fp:
        fp.write(generate_rust_file(mcu_ioregs))


if __name__ == '__main__':
    main()