extern crate avrvc;
extern crate stderrlog;

#[macro_use]
extern crate pretty_assertions;

mod common;

use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::core::CpuSignal;
use common::setup_emulator;
use common::run_emulator;
use common::setup_test;

#[test]
fn run_uart() {
    setup_test();

    let mut emulator = setup_emulator("sources/usart_out.c", &ATxmega128A4U, &vec![]);

    emulator.vm.debugger.trace = true;

    assert_eq!(run_emulator(&mut emulator, 1000), Some(CpuSignal::Break));
}