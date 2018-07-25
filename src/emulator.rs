use core::AvrVm;
use core::AvrVmInfo;
use models::register_gpio;
use models::usart::register_usarts;
use std::sync::Arc;
use std::sync::Mutex;
use models::usart::Usart;
use models::usart::Usarts;


pub struct AvrEmulator {
    pub vm: AvrVm,

    // peripherals
    pub usarts: Usarts
}

impl AvrEmulator {
    pub fn new(info: &AvrVmInfo) -> AvrEmulator {
        let mut vm = AvrVm::new(info);

        register_gpio(&mut vm);
        let usarts = register_usarts(&mut vm);

        AvrEmulator {
            vm,
            usarts
        }
    }

    pub fn from_name(name: &str) -> AvrEmulator {
        AvrEmulator::new(&AvrVmInfo::from_name(name))
    }
}