use models::AvrModel;
use core::AvrVm;
use core::AvrVmInfo;
use models::register_gpio;
use models::AvrMcu;
use models::usart::register_usart;

pub enum XmegaA4U {
    ATxmega16A4U,
    ATxmega32A4U,
    ATxmega64A4U,
    ATxmega128A4U,
}

impl AvrMcu for XmegaA4U {
    fn name(&self) -> &'static str {
        use models::xmega_au::XmegaA4U::*;

        match self {
            &ATxmega16A4U => "atxmega16a4u",
            &ATxmega32A4U => "atxmega32a4u",
            &ATxmega64A4U => "atxmega64a4u",
            &ATxmega128A4U => "atxmega128a4u",
        }
    }
}


impl AvrModel for XmegaA4U {
    fn create_vm(&self) -> AvrVm {
        let info = AvrVmInfo::from_name(self.name());
        let mut vm = AvrVm::new(&info);

        register_gpio(&mut vm);
        register_usart(&mut vm);

        vm.core.sp = info.ram.end - 1;
        vm
    }
}