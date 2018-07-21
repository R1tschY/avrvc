#![allow(dead_code)]

use avrvc::models::AvrMcu;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use avrvc::emulator::AvrEmulator;
use avrvc::executable::read_executable_file;
use avrvc::models::AvrModel;


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BinaryType {
    Elf,
    Hex,
    Binary,
}

impl BinaryType {
    pub fn to_str(&self) -> &'static str {
        use common::BinaryType::*;

        match *self {
            Elf => "elf32-avr",
            Hex => "ihex",
            Binary => "binary",
        }
    }

    pub fn file_extension(&self) -> &'static str {
        use common::BinaryType::*;

        match *self {
            Elf => "elf",
            Hex => "hex",
            Binary => "bin",
        }
    }
}


pub fn compile_binary(src: &Path, dest: &Path, mcu: &AvrMcu, flags: &[&str]) {
    let status = Command::new("avr-gcc")
        .arg(src)
        .arg("-o").arg(dest)
        .arg(format!("-mmcu={}", mcu.name()))
        .arg("-g")
        .args(flags)
        .status()
        .expect("executing compiler failed");

    if !status.success() {
        match status.code() {
            Some(code) => panic!("Compilation existed with status code: {}", code),
            None       => panic!("Compilation terminated by signal")
        }
    }
}

pub fn convert_binary(infile: &Path, intype: BinaryType, outfile: &Path, outtype: BinaryType) {
    let status = Command::new("avr-objcopy")
        .arg("-I").arg(intype.to_str())
        .arg("-O").arg(outtype.to_str())
        .arg(infile)
        .arg(outfile)
        .status()
        .expect("executing binary convertion failed");

    if !status.success() {
        match status.code() {
            Some(code) => panic!("Convertion existed with status code: {}", code),
            None       => panic!("Convertion terminated by signal")
        }
    }
}

/// compile test source and return path to compiled binary.
pub fn compile_test(srcfilename: &str, outtype: BinaryType, mcu: &AvrMcu, flags: &[&str]) -> PathBuf {
    let mut testsdir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    testsdir.push("tests");

    let mut elffile = testsdir.clone();
    elffile.push("build");
    elffile.push(srcfilename);
    elffile.set_extension("elf");

    let mut destination = elffile.clone();
    if outtype != BinaryType::Elf {
        destination.set_extension(outtype.file_extension());
    };

    let mut source = testsdir.clone();
    source.push(srcfilename);

    fs::create_dir_all(elffile.parent().unwrap()).expect("creation of build directory failed");

    compile_binary(&source, &elffile, mcu, flags);

    if outtype != BinaryType::Elf {
        convert_binary(&elffile, BinaryType::Elf, &destination, outtype);
    }

    destination
}


/// compile source and create emulator
pub fn setup_emulator(srcfilename: &str, model: &AvrModel, flags: &[&str]) -> AvrEmulator {
    let binary = compile_test(srcfilename, BinaryType::Binary, model.mcu(), flags);

    let bytes = read_executable_file(&binary);

    let mut emulator = model.create_emulator();
    emulator.vm.write_flash(0, &bytes);
    emulator
}