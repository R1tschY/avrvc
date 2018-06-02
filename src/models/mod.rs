use core::AvrVm;

pub mod xmega_au;
pub mod register_service;


/// model of avr controller type
pub trait AvrModel {

    fn create_vm(&self) -> AvrVm;

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
}