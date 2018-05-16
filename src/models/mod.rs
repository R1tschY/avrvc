use core::AvrVm;

pub mod xmega_au;


/// model of avr controller type
pub trait AvrModel {

    fn create_vm(&self) -> AvrVm;

}