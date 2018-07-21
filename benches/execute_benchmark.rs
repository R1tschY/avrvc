#![feature(test)]

extern crate avrvc;
extern crate test;
extern crate time;

#[macro_use]
extern crate pretty_assertions;

use avrvc::executable::read_executable_file;
use avrvc::tools::objdump::objdump;
use std::fs::File;
use std::io::Read;
use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::models::AvrModel;
use time::precise_time_s;
use std::path::Path;

#[test]
fn objdump_execute_benchmark() {
    let bytes = read_executable_file(Path::new("benches/execute_benchmark/main.bin"));
    let actual = objdump(&bytes) + "\n";

    let mut f = File::open("benches/execute_benchmark/main.S").expect("file not found");
    let mut expected = String::new();
    f.read_to_string(&mut expected).expect("something went wrong reading the file");

    // for pretty diff
    let actual_lines: Vec<_> = actual.split("\n").collect();
    let expected_lines: Vec<_> = expected.split("\n").collect();

    assert_eq!(expected_lines, actual_lines);
}

#[test]
fn run_execute_benchmark() {
    let bytes = read_executable_file(Path::new("benches/execute_benchmark/main.bin"));
    let _actual = objdump(&bytes) + "\n";

    let mut vm = ATxmega128A4U.create_vm();
    vm.write_flash(0, &bytes);
    // vm.debugger.trace = true;

    let start = precise_time_s();

    let mut executed_instr = 0;
    for i in 0..1000000000 {
        executed_instr = i;
        if let Err(_signal) = vm.step() {
            break;
        }
    }
    assert_eq!(executed_instr, 212000071);

    let end = precise_time_s();
    println!("Instructions: {} / {} s -> {:.2} MInstr/s",
             executed_instr, end - start, (executed_instr as f64 / (end - start)) / 1_000_000.);
    println!("Cycles: ~{} / {} s -> ~{:.2} MHz",
             vm.core.cycles, end - start, (vm.core.cycles as f64 / (end - start)) / 1_000_000.);
}