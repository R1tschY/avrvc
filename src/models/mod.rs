use core::AvrVm;
use emulator::AvrEmulator;

pub mod xmega_au;
pub mod register_service;
pub mod usart;
pub mod envmodel;


/// model of avr controller type
pub trait AvrModel {

    //#[deprecated(note="please use `create_emulator` instead")]
    fn create_vm(&self) -> AvrVm {
        self.create_emulator().vm
    }

    fn create_emulator(&self) -> AvrEmulator;

}

pub trait AvrMcu {

    fn name(&self) -> &'static str;

}


pub fn register_gpio(vm: &mut AvrVm) {
    let ioregs = vm.info.io_regs.clone();

    vm.register_io(
        ioregs["SREG"],
        Box::new(|core, _, _| core.read_sreg()),
        Box::new(|core, _, value| core.write_sreg(value))
    );

    vm.register_io(
        ioregs["SPL"],
        Box::new(|core, _, _| (core.sp & 0xFF) as u8),
        Box::new(|core, _, value| core.sp = (core.sp & !0xFF) | value as usize)
    );

    vm.register_io(
        ioregs["SPH"],
        Box::new(|core, _, _| (core.sp >> 8) as u8),
        Box::new(|core, _, value| core.sp = (core.sp & !0xFF00) | ((value as usize) << 8))
    );

    ioregs.get("RAMPD").map(|&rampd|
        vm.register_io(
            rampd,
            Box::new(|core, _, _| core.rampd),
            Box::new(|core, _, value| core.rampd = value)
        )
    );

    ioregs.get("RAMPX").map(|&rampx|
        vm.register_io(
            rampx,
            Box::new(|core, _, _| core.rampx),
            Box::new(|core, _, value| core.rampx = value)
        )
    );

    ioregs.get("RAMPY").map(|&rampy|
        vm.register_io(
            rampy,
            Box::new(|core, _, _| core.rampy),
            Box::new(|core, _, value| core.rampy = value)
        )
    );

    ioregs.get("RAMPZ").map(|&rampz|
        vm.register_io(
            rampz,
            Box::new(|core, _, _| core.rampz),
            Box::new(|core, _, value| core.rampz = value)
        )
    );
}