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
use common::run_emulator;

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

    assert_eq!(run_emulator(&emulator, 1000), Some(CpuSignal::Break));
}