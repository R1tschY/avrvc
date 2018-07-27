use core::AvrVm;
use std::collections::HashMap;
use std::sync::Arc;
use core::AvrCoreState;
use std::sync::Mutex;
use byte_convert::u8bits;
use byte_convert::bit_at;
use ::internals::signals::{Broadcast, BroadcastListener};


static USART_INDEXES: [&'static str; 8] = ["C0", "C1", "D0", "D1", "E0", "E1", "F0", "F1"];

pub type Usarts = HashMap<&'static str, Arc<Mutex<Usart>>>;
pub type UsartTxSignal = Broadcast<u8>;
pub type UsartTxConnection = BroadcastListener<u8>;

pub struct Usart {
    rx: u8,
    tx: u8,
//    baudrate: u16,
//    baudrate_scale: u8,
    rx_enable: bool,
    tx_enable: bool,
    data_empty: bool,
    tx_signal: UsartTxSignal,

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
            tx_signal: Broadcast::new(),
            index
        }
    }

    pub fn get_index(&self) -> &str { self.index }

    pub fn connect_to_tx(&mut self) -> UsartTxConnection {
        self.tx_signal.create_listener()
    }

    fn data_read(&mut self, core: &AvrCoreState, view: bool) -> u8 {
        0
    }

    fn data_write(&mut self, core: &mut AvrCoreState, value: u8) {
        if self.tx_enable {
            info!(
                target: "avrvc::usart",
                "USART {} Tx: 0x{:02x} {}",
                self.index,
                value,
                if value.is_ascii_graphic() { value as char } else { '?' });
            self.tx_signal.send(value);
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

    fn control_b_read(&mut self, core: &AvrCoreState, view: bool) -> u8 {
        0
    }

    fn control_b_write(&mut self, core: &mut AvrCoreState, value: u8) {
        self.rx_enable = bit_at(value, 4);
        self.tx_enable = bit_at(value, 3);
        info!(
            target: "avrvc::usart",
            "USART {} Control B: RXEN={} TXEN={}",
            self.index, self.rx_enable as u8, self.tx_enable as u8);
    }
}


pub fn register_usarts(vm: &mut AvrVm) -> Usarts {
    USART_INDEXES.iter().filter_map(
        |&index| register_one_usart(vm, index).map(|usart| (index, usart))
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
    let usart5 = Arc::clone(&usart);
    let usart6 = Arc::clone(&usart);

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
    vm.register_io(
        ioregs[&*format!("USART{}_CTRLB", index)],
        Box::new(move |core, _, view| usart5.lock().unwrap().control_b_read(core, view)),
        Box::new(move |core, _, value| usart6.lock().unwrap().control_b_write(core, value))
    );

    Some(usart)
}