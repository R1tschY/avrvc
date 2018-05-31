
// GENERATED - DO NOT EDIT!

use std::collections::HashMap;

pub type IoRegAddrs = HashMap<&'static str, usize>;

pub struct McuIoRegistersService {
    mcus: HashMap<&'static str, HashMap<&'static str, usize>>
}

impl McuIoRegistersService {
    pub fn new() -> McuIoRegistersService {
        let mut service = McuIoRegistersService { mcus: HashMap::new() };

        let mut mcu_atmega8: HashMap<&'static str, usize> = HashMap::new();
        mcu_atmega8.insert("SPL", 0x5d);
        mcu_atmega8.insert("PORTD", 0x32);
        mcu_atmega8.insert("DDRC", 0x34);
        mcu_atmega8.insert("SREG", 0x5f);
        mcu_atmega8.insert("SPH", 0x5e);
        mcu_atmega8.insert("PORTC", 0x35);
        mcu_atmega8.insert("PINC", 0x33);
        mcu_atmega8.insert("DDRB", 0x37);
        mcu_atmega8.insert("PINB", 0x36);
        mcu_atmega8.insert("DDRD", 0x31);
        mcu_atmega8.insert("PIND", 0x30);
        mcu_atmega8.insert("PORTB", 0x38);
        service.mcus.insert("atmega8", mcu_atmega8);
        
        let mut mcu_atmega16: HashMap<&'static str, usize> = HashMap::new();
        mcu_atmega16.insert("SPL", 0x5d);
        mcu_atmega16.insert("PORTD", 0x32);
        mcu_atmega16.insert("DDRC", 0x34);
        mcu_atmega16.insert("PINA", 0x39);
        mcu_atmega16.insert("SREG", 0x5f);
        mcu_atmega16.insert("SPH", 0x5e);
        mcu_atmega16.insert("PORTC", 0x35);
        mcu_atmega16.insert("PINC", 0x33);
        mcu_atmega16.insert("DDRB", 0x37);
        mcu_atmega16.insert("PORTA", 0x3b);
        mcu_atmega16.insert("PINB", 0x36);
        mcu_atmega16.insert("DDRA", 0x3a);
        mcu_atmega16.insert("DDRD", 0x31);
        mcu_atmega16.insert("PIND", 0x30);
        mcu_atmega16.insert("PORTB", 0x38);
        service.mcus.insert("atmega16", mcu_atmega16);
        
        let mut mcu_atxmega16a4u: HashMap<&'static str, usize> = HashMap::new();
        mcu_atxmega16a4u.insert("SPL", 0x3d);
        mcu_atxmega16a4u.insert("PORTD", 0x660);
        mcu_atxmega16a4u.insert("SREG", 0x3f);
        mcu_atxmega16a4u.insert("SPH", 0x3e);
        mcu_atxmega16a4u.insert("PORTC", 0x640);
        mcu_atxmega16a4u.insert("PORTA", 0x600);
        mcu_atxmega16a4u.insert("PORTB", 0x620);
        service.mcus.insert("atxmega16a4u", mcu_atxmega16a4u);
        
        let mut mcu_atxmega32a4u: HashMap<&'static str, usize> = HashMap::new();
        mcu_atxmega32a4u.insert("SPL", 0x3d);
        mcu_atxmega32a4u.insert("PORTD", 0x660);
        mcu_atxmega32a4u.insert("SREG", 0x3f);
        mcu_atxmega32a4u.insert("SPH", 0x3e);
        mcu_atxmega32a4u.insert("PORTC", 0x640);
        mcu_atxmega32a4u.insert("PORTA", 0x600);
        mcu_atxmega32a4u.insert("PORTB", 0x620);
        service.mcus.insert("atxmega32a4u", mcu_atxmega32a4u);
        
        let mut mcu_atxmega64a4u: HashMap<&'static str, usize> = HashMap::new();
        mcu_atxmega64a4u.insert("SPL", 0x3d);
        mcu_atxmega64a4u.insert("PORTD", 0x660);
        mcu_atxmega64a4u.insert("SREG", 0x3f);
        mcu_atxmega64a4u.insert("SPH", 0x3e);
        mcu_atxmega64a4u.insert("PORTC", 0x640);
        mcu_atxmega64a4u.insert("PORTA", 0x600);
        mcu_atxmega64a4u.insert("PORTB", 0x620);
        service.mcus.insert("atxmega64a4u", mcu_atxmega64a4u);
        
        let mut mcu_atxmega128a4u: HashMap<&'static str, usize> = HashMap::new();
        mcu_atxmega128a4u.insert("SPL", 0x3d);
        mcu_atxmega128a4u.insert("PORTD", 0x660);
        mcu_atxmega128a4u.insert("SREG", 0x3f);
        mcu_atxmega128a4u.insert("SPH", 0x3e);
        mcu_atxmega128a4u.insert("PORTC", 0x640);
        mcu_atxmega128a4u.insert("PORTA", 0x600);
        mcu_atxmega128a4u.insert("PORTB", 0x620);
        service.mcus.insert("atxmega128a4u", mcu_atxmega128a4u);
        

        service
    }

    pub fn get_mcu_registers(&self, mcu: &str) -> Option<&HashMap<&'static str, usize>> {
        self.mcus.get(mcu)
    }
}
