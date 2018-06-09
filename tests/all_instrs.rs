extern crate avrvc;

#[macro_use]
extern crate pretty_assertions;

use avrvc::executable::read_executable_file;
use avrvc::tools::objdump::objdump;
use std::fs;

#[test]
fn all_instrs() {
    let bytes = read_executable_file("tests/all_instrs/all_instrs.bin");
    let actual = objdump(&bytes) + "\n";

    let expected = fs::read_to_string("tests/all_instrs/main.S").unwrap();

    // for pretty diff
    let actual_lines: Vec<_> = actual.split("\n").collect();
    let expected_lines: Vec<_> = expected.split("\n").collect();

    let mut i = 0;
    for (&act, &exp) in actual_lines.iter().zip(expected_lines.iter()) {
        if act != "invalid" { // TODO: remove if we support all
            assert_eq!(act, exp, "opcode: 0b{:b}", i);
        }

        if exp.starts_with(".word") {
            assert_eq!(act, "invalid");
        }
        i += 1;
    }

    assert_eq!(actual_lines.len(), 65537);
    assert_eq!(expected_lines.len(), 65537);


    let decoded_act = actual_lines.iter().filter(|&&s| s != "invalid").count();
    let decoded_exp = expected_lines.iter().filter(|&&s| !s.starts_with(".word")).count();

    assert_eq!(decoded_exp, 63983);
    println!("actual: decoded {} out of {}", decoded_act, 65537);
    println!("expected: decoded {} out of {}", decoded_exp, 65537);
}