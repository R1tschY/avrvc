extern crate avrvc;

#[macro_use]
extern crate pretty_assertions;

use avrvc::executable::read_executable_file;
use avrvc::tools::objdump::objdump;
use std::fs::File;
use std::io::Read;
use avrvc::core::CpuSignal;
use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::models::AvrModel;
use std::path::Path;

#[test]
fn objdump_nothing() {
    let bytes = read_executable_file(Path::new("tests/nothing/main.bin"));
    let actual = objdump(&bytes) + "\n";

    let mut f = File::open("tests/nothing/main.S").expect("file not found");
    let mut expected = String::new();
    f.read_to_string(&mut expected).expect("something went wrong reading the file");

    // for pretty diff
    let actual_lines: Vec<_> = actual.split("\n").collect();
    let expected_lines: Vec<_> = expected.split("\n").collect();

    assert_eq!(expected_lines, actual_lines);
}


#[test]
fn run_nothing() {
    let bytes = read_executable_file(Path::new("tests/nothing/main.bin"));
    let _actual = objdump(&bytes) + "\n";

    let mut vm = ATxmega128A4U.create_vm();
    vm.write_flash(0, &bytes);

    vm.debugger.trace = true;
    vm.debugger.add_breakpoint(0x11A);

    for _i in 0..100 {
        if let Err(signal) = vm.step() {
            assert_eq!(signal, CpuSignal::Break);
            break;
        }
    }

    assert_eq!(vm.read_unchecked(0x3FFF - 0, true), 0x0c);
    assert_eq!(vm.read_unchecked(0x3FFF - 1, true), 0x01);
    assert_eq!(vm.read_unchecked(0x3FFF - 2, true), 0x00);
    assert_eq!(vm.read_unchecked(0x3FFF - 3, true), 0xff);
    assert_eq!(vm.read_unchecked(0x3FFF - 4, true), 0x3f);
    assert_eq!(vm.read_unchecked(0x3FFF - 5, true), 0x00);
    assert_eq!(vm.core.pc, 0x11A);
    assert_eq!(vm.core.sp, 0x3fff);
    assert!(vm.core.cycles >= 37 && vm.core.cycles <= 40);
    assert_eq!(vm.core.read_reg(24), 0x2A);
    assert_eq!(vm.core.read_reg(28), 0xFF);
    assert_eq!(vm.core.read_reg(29), 0x3F);
    assert_eq!(vm.core.read_x(), 0x0000);
    assert_eq!(vm.core.read_y(), 0x3FFF);
    assert_eq!(vm.core.read_z(), 0x0000);
    assert_eq!(vm.core.read_sreg(), 0x00);
}