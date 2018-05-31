extern crate avrvc;

#[macro_use]
extern crate pretty_assertions;

use avrvc::executable::read_executable_file;
use avrvc::tools::objdump::objdump;
use std::fs::File;
use std::io::Read;

#[test]
fn all_instrs() {
    let bytes = read_executable_file("tests/all_instrs/all_instrs.bin");
    let actual = objdump(&bytes) + "\n";

    let mut f = File::open("tests/all_instrs/main.S").expect("file not found");
    let mut expected = String::new();
    f.read_to_string(&mut expected).expect("something went wrong reading the file");

    // for pretty diff
    let actual_lines: Vec<_> = actual.split("\n").collect();
    let expected_lines: Vec<_> = expected.split("\n").collect();

    for (&act, &exp) in actual_lines.iter().zip(expected_lines.iter()) {
        if act != "invalid" { // TODO: remove if we support all
            assert_eq!(act, exp);
        }
    }

    assert_eq!(actual_lines.len(), 65537);
    assert_eq!(expected_lines.len(), 65537);


    let decoded_act = actual_lines.iter().filter(|&&s| s != "invalid").count();
    let decoded_exp = expected_lines.iter().filter(|&&s| !s.starts_with(".word")).count();

    println!("actual: decoded {} out of {}", decoded_act, 65537);
    println!("expected: decoded {} out of {}", decoded_exp, 65537);
}