use models::AvrModel;
use core::AvrVm;
use core::AvrVmInfo;

pub enum XmegaA4U {
    ATxmega16A4U,
    ATxmega32A4U,
    ATxmega64A4U,
    ATxmega128A4U,
}

impl AvrModel for XmegaA4U {
    fn create_vm(&self) -> AvrVm {
        use models::xmega_au::XmegaA4U::*;

        let flash_bytes = match self {
            &ATxmega16A4U => 0x27FF + 1,
            &ATxmega32A4U => 0x47FF + 1,
            &ATxmega64A4U => 0x87FF + 1,
            &ATxmega128A4U => 0x10FFF + 1,
        };

        let ram_end = match self {
            &ATxmega16A4U => 0x27FF + 1,
            &ATxmega32A4U => 0x2FFF + 1,
            &ATxmega64A4U => 0x2FFF + 1,
            &ATxmega128A4U => 0x3FFF + 1,
        };

        let eeprom_end = match self {
            &ATxmega16A4U => 0x13FF + 1,
            &ATxmega32A4U => 0x13FF + 1,
            &ATxmega64A4U => 0x17FF + 1,
            &ATxmega128A4U => 0x17FF + 1,
        };

        let info = AvrVmInfo{
            pc_bytes: 3,
            xmega: true,
            flash_bytes,
            ios: 0xFFF + 1,
            ram: 0x2000..ram_end,
            eeprom: 0x1000..eeprom_end
        };

        let mut vm = AvrVm::new(&info);
        vm.sp = ram_end - 1;
        vm
    }
}