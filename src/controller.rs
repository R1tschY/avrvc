use core::{AvrVm, CpuSignal};

pub struct AvrController {
    pub core: AvrVm
}

impl AvrController {

    pub fn step(&mut self) -> Result<(), CpuSignal> {
        self.core.step()
    }

}