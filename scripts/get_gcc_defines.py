#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import subprocess
from argparse import ArgumentParser, REMAINDER
from tempfile import NamedTemporaryFile
from typing import List


def get_compiler_output(compiler: str, flags: List[str]) -> str:
    with NamedTemporaryFile(suffix=".c") as fp:
        return subprocess.run([
            compiler
        ] + flags + [
            "-E", "-P", "-v", "-dD", fp.name
        ], check=True, stderr=subprocess.PIPE).stderr.decode("utf-8")


def main():
    parser = ArgumentParser(description='Get compiler defines')
    parser.add_argument(
        'compiler', metavar='COMPILER', help='an gcc compatible compiler',
        default="avr-gcc"
    )
    parser.add_argument(
        'flags', metavar='FLAG', type=str, nargs=REMAINDER,
        help='compiler flags')
    args = parser.parse_args()

    print(
        get_compiler_output(args.compiler, args.flags)
    )


if __name__ == '__main__':
    main()
