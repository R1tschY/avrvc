extern crate avrvc;
extern crate stderrlog;

#[macro_use]
extern crate pretty_assertions;

mod common;

use avrvc::executable::read_executable_file;
use avrvc::tools::objdump::objdump;
use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::core::CpuSignal;
use avrvc::models::AvrModel;
use stderrlog::Timestamp;
use common::compile_test;
use common::setup_emulator;

#[test]
fn run_uart() {
    use common::BinaryType::*;

    stderrlog::new()
        .verbosity(3)
        .timestamp(Timestamp::Off)
        .init()
        .unwrap();

    let mut emulator = setup_emulator("sources/usart_out.c", &ATxmega128A4U, &vec![]);

    emulator.vm.debugger.trace = true;

    for _i in 0..1000 {
        if let Err(signal) = emulator.vm.step() {
            assert_eq!(signal, CpuSignal::Break);
            break;
        }
    }

//    assert_eq!(emulator.read_mem(0x3FFF - 0), 0x0c);
//    assert_eq!(emulator.read_mem(0x3FFF - 1), 0x01);
//    assert_eq!(emulator.read_mem(0x3FFF - 2), 0x00);
//    assert_eq!(emulator.read_mem(0x3FFF - 3), 0xff);
//    assert_eq!(emulator.read_mem(0x3FFF - 4), 0x3f);
//    assert_eq!(emulator.read_mem(0x3FFF - 5), 0x00);
//    assert_eq!(emulator.pc, 0x11A);
//    assert_eq!(emulator.sp, 0x3fff);
//    assert!(emulator.cycles >= 37 && emulator.cycles <= 40);
//    assert_eq!(emulator.read_reg(24), 0x2A);
//    assert_eq!(emulator.read_reg(28), 0xFF);
//    assert_eq!(emulator.read_reg(29), 0x3F);
//    assert_eq!(emulator.read_x(), 0x0000);
//    assert_eq!(emulator.read_y(), 0x3FFF);
//    assert_eq!(emulator.read_z(), 0x0000);
//    assert_eq!(emulator.read_sreg(), 0x02);
}