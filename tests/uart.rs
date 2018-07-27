extern crate avrvc;
extern crate stderrlog;
extern crate futures;

#[macro_use]
extern crate pretty_assertions;

mod common;

use avrvc::models::xmega_au::XmegaA4U::ATxmega128A4U;
use avrvc::core::CpuSignal;
use common::setup_emulator;
use common::run_emulator;
use common::setup_test;
use futures::stream::Stream;
use futures::Future;


#[test]
fn run_uart() {
    setup_test();

    let mut emulator = setup_emulator("sources/usart_out.c", &ATxmega128A4U, &vec![]);

//    emulator.vm.debugger.trace = true;
    let tx = emulator.usarts["C0"].lock().unwrap().connect_to_tx();

    assert_eq!(run_emulator(&mut emulator, 1000), Some(CpuSignal::Break));
    drop(emulator);

    assert_eq!(tx.collect().wait().unwrap(), b"Hello World!\n");
}