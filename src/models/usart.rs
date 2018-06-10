use core::AvrVm;
use std::collections::HashMap;
use std::sync::Arc;
use core::AvrCoreState;
use std::sync::Mutex;
use byte_convert::u8bits;


static USART_INDEXES: [&'static str; 8] = ["C0", "C1", "D0", "D1", "E0", "E1", "F0", "F1"];

struct Usart {
    pub rx: u8,
    pub tx: u8,
    pub baudrate: u16,
    pub baudrate_scale: u8,
    pub rx_enable: bool,
    pub tx_enable: bool,
    pub data_empty: bool,

    pub index: &'static str
}

impl Usart {
    pub fn new(index: &'static str) -> Usart {
        Usart {
            rx: 0,
            tx: 0,
            baudrate: 0,
            baudrate_scale: 0,
            rx_enable: false,
            tx_enable: false,
            data_empty: true,
            index
        }
    }

    fn data_read(&mut self, core: &AvrCoreState, view: bool) -> u8 {
        0
    }

    fn data_write(&mut self, core: &mut AvrCoreState, value: u8) {
        info!(target: "avrvc::usart", "USART Tx {}: 0x{:02x} {}",
              self.index, value,
              if value.is_ascii_graphic() { value as char } else { '?' });
        if self.rx_enable {
            // self.env.send_event(UartTx(value))
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


pub fn register_usart(vm: &mut AvrVm) {
    let ioregs = vm.info.io_regs.clone();

    for index in USART_INDEXES.iter() {
        register_one_usart(vm, &ioregs, index);
    }
}

fn register_one_usart(vm: &mut AvrVm, ioregs: &HashMap<&str, usize>, index: &'static str) {
    if !ioregs.contains_key(&*format!("USART{}_DATA", index)) {
        return
    }

    let usart1 = Arc::new(Mutex::new(Usart::new(index)));
    let usart2 = Arc::clone(&usart1);
    let usart3 = Arc::clone(&usart1);
    let usart4 = Arc::clone(&usart1);

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
}