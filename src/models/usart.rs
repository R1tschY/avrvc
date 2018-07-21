use core::AvrVm;
use std::collections::HashMap;
use std::sync::Arc;
use core::AvrCoreState;
use std::sync::Mutex;
use byte_convert::u8bits;


static USART_INDEXES: [&'static str; 8] = ["C0", "C1", "D0", "D1", "E0", "E1", "F0", "F1"];

pub struct Usart {
    rx: u8,
    tx: u8,
//    baudrate: u16,
//    baudrate_scale: u8,
    rx_enable: bool,
    tx_enable: bool,
    data_empty: bool,

    index: &'static str
}

impl Usart {
    pub fn new(index: &'static str) -> Usart {
        Usart {
            rx: 0,
            tx: 0,
//            baudrate: 0,
//            baudrate_scale: 0,
            rx_enable: false,
            tx_enable: false,
            data_empty: true,
            index
        }
    }

    pub fn get_index(&self) -> &str { self.index }

    fn data_read(&mut self, core: &AvrCoreState, view: bool) -> u8 {
        0
    }

    fn data_write(&mut self, core: &mut AvrCoreState, value: u8) {
        info!(target: "avrvc::usart", "USART Tx {}: 0x{:02x} {}",
              self.index, value,
              if value.is_ascii_graphic() { value as char } else { '?' });
        if self.rx_enable {
            // self.env.send_event(UartTx(index, value))
        }
    }

    fn status_read(&mut self, core: &AvrCoreState, view: bool) -> u8 {
        u8bits(
            false, // RXCIF: Receive Complete Interrupt Flag
            false, // TXCIF: Transmit Complete Interrupt Flag
            self.data_empty, // DREIF: Data Register Empty Flag
            false, // FERR: Frame Error

            false, // BUFOVF: Buffer Overflow
            false, // PERR: Parity Error
            false, // Reserved
            false  // RXB8: Receive Bit 8
        )
    }

    fn status_write(&mut self, core: &mut AvrCoreState, value: u8) {

    }
}


pub fn register_usarts(vm: &mut AvrVm) -> Vec<Arc<Mutex<Usart>>> {
    USART_INDEXES.iter().flat_map(
        |index| register_one_usart(vm, index)
    ).collect()
}

fn register_one_usart(
    vm: &mut AvrVm, index: &'static str
) -> Option<Arc<Mutex<Usart>>> {
    if !vm.info.io_regs.contains_key(&*format!("USART{}_DATA", index)) {
        return None
    }

    let ioregs = vm.info.io_regs.clone();

    let usart = Arc::new(Mutex::new(Usart::new(index)));
    let usart1 = Arc::clone(&usart);
    let usart2 = Arc::clone(&usart);
    let usart3 = Arc::clone(&usart);
    let usart4 = Arc::clone(&usart);

    vm.register_io(
        ioregs[&*format!("USART{}_DATA", index)],
        Box::new(move |core, _, view| usart1.lock().unwrap().data_read(core, view)),
        Box::new(move |core, _, value| usart2.lock().unwrap().data_write(core, value))
    );
    vm.register_io(
        ioregs[&*format!("USART{}_STATUS", index)],
        Box::new(move |core, _, view| usart3.lock().unwrap().status_read(core, view)),
        Box::new(move |core, _, value| usart4.lock().unwrap().status_write(core, value))
    );

    Some(usart)
}