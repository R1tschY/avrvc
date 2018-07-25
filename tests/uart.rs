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
use avrvc::models::usart::UsartTxConnection;


fn read_tx_buffer(tx: &mut UsartTxConnection) -> String {
    let mut result: Vec<u8> = vec![];
    loop {
        let byte = match tx.try_recv() {
            Ok(byte) => byte,
            Err(err) => return String::from_utf8_lossy(&result).to_string()
        };
        result.push(byte);
    }
}

#[test]
fn run_uart() {
    setup_test();

    let mut emulator = setup_emulator("sources/usart_out.c", &ATxmega128A4U, &vec![]);

//    emulator.vm.debugger.trace = true;
    let mut tx = emulator.usarts["C0"].lock().unwrap().connect_to_tx();

    assert_eq!(run_emulator(&mut emulator, 1000), Some(CpuSignal::Break));
    assert_eq!(read_tx_buffer(&mut tx), "Hello World!\n")
}