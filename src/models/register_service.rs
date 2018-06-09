
// GENERATED - DO NOT EDIT!

use std::collections::HashMap;

pub type IoRegAddrs = HashMap<&'static str, usize>;

pub struct McuIoRegistersService {
    mcus: HashMap<&'static str, IoRegAddrs>
}

impl McuIoRegistersService {
    pub fn new() -> McuIoRegistersService {
        let mut service = McuIoRegistersService { mcus: HashMap::new() };

        let mut mcu_atmega8: IoRegAddrs = HashMap::new();
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
        mcu_atmega8.insert("#FLASHEND", 0x1fff);
        mcu_atmega8.insert("#__AVR_2_BYTE_PC__", 0x1);
        mcu_atmega8.insert("#RAMEND", 0x45f);
        mcu_atmega8.insert("#SPM_PAGESIZE", 0x40);
        mcu_atmega8.insert("#RAMSTART", 0x60);
        mcu_atmega8.insert("#__AVR_ARCH__", 0x4);
        service.mcus.insert("atmega8", mcu_atmega8);
        
        let mut mcu_atmega16: IoRegAddrs = HashMap::new();
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
        mcu_atmega16.insert("#FLASHEND", 0x3fff);
        mcu_atmega16.insert("#__AVR_MEGA__", 0x1);
        mcu_atmega16.insert("#__AVR_2_BYTE_PC__", 0x1);
        mcu_atmega16.insert("#RAMEND", 0x45f);
        mcu_atmega16.insert("#SPM_PAGESIZE", 0x80);
        mcu_atmega16.insert("#RAMSTART", 0x60);
        mcu_atmega16.insert("#__AVR_ARCH__", 0x5);
        service.mcus.insert("atmega16", mcu_atmega16);
        
        let mut mcu_atxmega16a4u: IoRegAddrs = HashMap::new();
        mcu_atxmega16a4u.insert("USARTE0_CTRLB", 0xaa4);
        mcu_atxmega16a4u.insert("SPL", 0x3d);
        mcu_atxmega16a4u.insert("USARTE0_DATA", 0xaa0);
        mcu_atxmega16a4u.insert("PORTD", 0x660);
        mcu_atxmega16a4u.insert("USARTC1_STATUS", 0x8b1);
        mcu_atxmega16a4u.insert("USARTD0_BAUDCTRLB", 0x9a7);
        mcu_atxmega16a4u.insert("SREG", 0x3f);
        mcu_atxmega16a4u.insert("SPH", 0x3e);
        mcu_atxmega16a4u.insert("PORTC", 0x640);
        mcu_atxmega16a4u.insert("USARTC0_CTRLC", 0x8a5);
        mcu_atxmega16a4u.insert("USARTD0_CTRLB", 0x9a4);
        mcu_atxmega16a4u.insert("USARTE0_BAUDCTRLB", 0xaa7);
        mcu_atxmega16a4u.insert("USARTD1_CTRLC", 0x9b5);
        mcu_atxmega16a4u.insert("USARTD0_BAUDCTRLA", 0x9a6);
        mcu_atxmega16a4u.insert("USARTD1_CTRLA", 0x9b3);
        mcu_atxmega16a4u.insert("USARTC1_DATA", 0x8b0);
        mcu_atxmega16a4u.insert("USARTE0_CTRLC", 0xaa5);
        mcu_atxmega16a4u.insert("RAMPX", 0x39);
        mcu_atxmega16a4u.insert("USARTC0_CTRLA", 0x8a3);
        mcu_atxmega16a4u.insert("USARTC1_CTRLB", 0x8b4);
        mcu_atxmega16a4u.insert("USARTD0_CTRLA", 0x9a3);
        mcu_atxmega16a4u.insert("USARTC1_BAUDCTRLB", 0x8b7);
        mcu_atxmega16a4u.insert("USARTD0_DATA", 0x9a0);
        mcu_atxmega16a4u.insert("USARTD1_DATA", 0x9b0);
        mcu_atxmega16a4u.insert("RAMPD", 0x38);
        mcu_atxmega16a4u.insert("USARTE0_STATUS", 0xaa1);
        mcu_atxmega16a4u.insert("USARTC1_CTRLC", 0x8b5);
        mcu_atxmega16a4u.insert("USARTD1_BAUDCTRLB", 0x9b7);
        mcu_atxmega16a4u.insert("USARTD0_CTRLC", 0x9a5);
        mcu_atxmega16a4u.insert("USARTD1_BAUDCTRLA", 0x9b6);
        mcu_atxmega16a4u.insert("USARTE0_CTRLA", 0xaa3);
        mcu_atxmega16a4u.insert("PORTA", 0x600);
        mcu_atxmega16a4u.insert("USARTC1_BAUDCTRLA", 0x8b6);
        mcu_atxmega16a4u.insert("RAMPZ", 0x3b);
        mcu_atxmega16a4u.insert("USARTC0_BAUDCTRLB", 0x8a7);
        mcu_atxmega16a4u.insert("USARTD1_CTRLB", 0x9b4);
        mcu_atxmega16a4u.insert("USARTC0_STATUS", 0x8a1);
        mcu_atxmega16a4u.insert("USARTC0_BAUDCTRLA", 0x8a6);
        mcu_atxmega16a4u.insert("USARTD1_STATUS", 0x9b1);
        mcu_atxmega16a4u.insert("USARTC0_DATA", 0x8a0);
        mcu_atxmega16a4u.insert("RAMPY", 0x3a);
        mcu_atxmega16a4u.insert("PORTB", 0x620);
        mcu_atxmega16a4u.insert("USARTC1_CTRLA", 0x8b3);
        mcu_atxmega16a4u.insert("USARTC0_CTRLB", 0x8a4);
        mcu_atxmega16a4u.insert("USARTE0_BAUDCTRLA", 0xaa6);
        mcu_atxmega16a4u.insert("USARTD0_STATUS", 0x9a1);
        mcu_atxmega16a4u.insert("#FLASHEND", 0x4fff);
        mcu_atxmega16a4u.insert("#__AVR_MEGA__", 0x1);
        mcu_atxmega16a4u.insert("#IO_SIZE", 0x1000);
        mcu_atxmega16a4u.insert("#__AVR_XMEGA__", 0x1);
        mcu_atxmega16a4u.insert("#__AVR_2_BYTE_PC__", 0x1);
        mcu_atxmega16a4u.insert("#RAMEND", 0x27ff);
        mcu_atxmega16a4u.insert("#MAPPED_EEPROM_END", 0x13ff);
        mcu_atxmega16a4u.insert("#SPM_PAGESIZE", 0x100);
        mcu_atxmega16a4u.insert("#MAPPED_EEPROM_START", 0x1000);
        mcu_atxmega16a4u.insert("#RAMSTART", 0x2000);
        mcu_atxmega16a4u.insert("#__AVR_ARCH__", 0x66);
        service.mcus.insert("atxmega16a4u", mcu_atxmega16a4u);
        
        let mut mcu_atxmega32a4u: IoRegAddrs = HashMap::new();
        mcu_atxmega32a4u.insert("USARTE0_CTRLB", 0xaa4);
        mcu_atxmega32a4u.insert("SPL", 0x3d);
        mcu_atxmega32a4u.insert("USARTE0_DATA", 0xaa0);
        mcu_atxmega32a4u.insert("PORTD", 0x660);
        mcu_atxmega32a4u.insert("USARTC1_STATUS", 0x8b1);
        mcu_atxmega32a4u.insert("USARTD0_BAUDCTRLB", 0x9a7);
        mcu_atxmega32a4u.insert("SREG", 0x3f);
        mcu_atxmega32a4u.insert("SPH", 0x3e);
        mcu_atxmega32a4u.insert("PORTC", 0x640);
        mcu_atxmega32a4u.insert("USARTC0_CTRLC", 0x8a5);
        mcu_atxmega32a4u.insert("USARTD0_CTRLB", 0x9a4);
        mcu_atxmega32a4u.insert("USARTE0_BAUDCTRLB", 0xaa7);
        mcu_atxmega32a4u.insert("USARTD1_CTRLC", 0x9b5);
        mcu_atxmega32a4u.insert("USARTD0_BAUDCTRLA", 0x9a6);
        mcu_atxmega32a4u.insert("USARTD1_CTRLA", 0x9b3);
        mcu_atxmega32a4u.insert("USARTC1_DATA", 0x8b0);
        mcu_atxmega32a4u.insert("USARTE0_CTRLC", 0xaa5);
        mcu_atxmega32a4u.insert("RAMPX", 0x39);
        mcu_atxmega32a4u.insert("USARTC0_CTRLA", 0x8a3);
        mcu_atxmega32a4u.insert("USARTC1_CTRLB", 0x8b4);
        mcu_atxmega32a4u.insert("USARTD0_CTRLA", 0x9a3);
        mcu_atxmega32a4u.insert("USARTC1_BAUDCTRLB", 0x8b7);
        mcu_atxmega32a4u.insert("USARTD0_DATA", 0x9a0);
        mcu_atxmega32a4u.insert("USARTD1_DATA", 0x9b0);
        mcu_atxmega32a4u.insert("RAMPD", 0x38);
        mcu_atxmega32a4u.insert("USARTE0_STATUS", 0xaa1);
        mcu_atxmega32a4u.insert("USARTC1_CTRLC", 0x8b5);
        mcu_atxmega32a4u.insert("USARTD1_BAUDCTRLB", 0x9b7);
        mcu_atxmega32a4u.insert("USARTD0_CTRLC", 0x9a5);
        mcu_atxmega32a4u.insert("USARTD1_BAUDCTRLA", 0x9b6);
        mcu_atxmega32a4u.insert("USARTE0_CTRLA", 0xaa3);
        mcu_atxmega32a4u.insert("PORTA", 0x600);
        mcu_atxmega32a4u.insert("USARTC1_BAUDCTRLA", 0x8b6);
        mcu_atxmega32a4u.insert("RAMPZ", 0x3b);
        mcu_atxmega32a4u.insert("USARTC0_BAUDCTRLB", 0x8a7);
        mcu_atxmega32a4u.insert("USARTD1_CTRLB", 0x9b4);
        mcu_atxmega32a4u.insert("USARTC0_STATUS", 0x8a1);
        mcu_atxmega32a4u.insert("USARTC0_BAUDCTRLA", 0x8a6);
        mcu_atxmega32a4u.insert("USARTD1_STATUS", 0x9b1);
        mcu_atxmega32a4u.insert("USARTC0_DATA", 0x8a0);
        mcu_atxmega32a4u.insert("RAMPY", 0x3a);
        mcu_atxmega32a4u.insert("PORTB", 0x620);
        mcu_atxmega32a4u.insert("USARTC1_CTRLA", 0x8b3);
        mcu_atxmega32a4u.insert("USARTC0_CTRLB", 0x8a4);
        mcu_atxmega32a4u.insert("USARTE0_BAUDCTRLA", 0xaa6);
        mcu_atxmega32a4u.insert("USARTD0_STATUS", 0x9a1);
        mcu_atxmega32a4u.insert("#FLASHEND", 0x8fff);
        mcu_atxmega32a4u.insert("#__AVR_MEGA__", 0x1);
        mcu_atxmega32a4u.insert("#IO_SIZE", 0x1000);
        mcu_atxmega32a4u.insert("#__AVR_XMEGA__", 0x1);
        mcu_atxmega32a4u.insert("#__AVR_2_BYTE_PC__", 0x1);
        mcu_atxmega32a4u.insert("#RAMEND", 0x2fff);
        mcu_atxmega32a4u.insert("#MAPPED_EEPROM_END", 0x13ff);
        mcu_atxmega32a4u.insert("#SPM_PAGESIZE", 0x100);
        mcu_atxmega32a4u.insert("#MAPPED_EEPROM_START", 0x1000);
        mcu_atxmega32a4u.insert("#RAMSTART", 0x2000);
        mcu_atxmega32a4u.insert("#__AVR_ARCH__", 0x66);
        service.mcus.insert("atxmega32a4u", mcu_atxmega32a4u);
        
        let mut mcu_atxmega64a4u: IoRegAddrs = HashMap::new();
        mcu_atxmega64a4u.insert("USARTE0_CTRLB", 0xaa4);
        mcu_atxmega64a4u.insert("SPL", 0x3d);
        mcu_atxmega64a4u.insert("USARTE0_DATA", 0xaa0);
        mcu_atxmega64a4u.insert("PORTD", 0x660);
        mcu_atxmega64a4u.insert("USARTC1_STATUS", 0x8b1);
        mcu_atxmega64a4u.insert("USARTD0_BAUDCTRLB", 0x9a7);
        mcu_atxmega64a4u.insert("SREG", 0x3f);
        mcu_atxmega64a4u.insert("SPH", 0x3e);
        mcu_atxmega64a4u.insert("PORTC", 0x640);
        mcu_atxmega64a4u.insert("USARTC0_CTRLC", 0x8a5);
        mcu_atxmega64a4u.insert("USARTD0_CTRLB", 0x9a4);
        mcu_atxmega64a4u.insert("USARTE0_BAUDCTRLB", 0xaa7);
        mcu_atxmega64a4u.insert("USARTD1_CTRLC", 0x9b5);
        mcu_atxmega64a4u.insert("USARTD0_BAUDCTRLA", 0x9a6);
        mcu_atxmega64a4u.insert("USARTD1_CTRLA", 0x9b3);
        mcu_atxmega64a4u.insert("USARTC1_DATA", 0x8b0);
        mcu_atxmega64a4u.insert("USARTE0_CTRLC", 0xaa5);
        mcu_atxmega64a4u.insert("RAMPX", 0x39);
        mcu_atxmega64a4u.insert("USARTC0_CTRLA", 0x8a3);
        mcu_atxmega64a4u.insert("USARTC1_CTRLB", 0x8b4);
        mcu_atxmega64a4u.insert("USARTD0_CTRLA", 0x9a3);
        mcu_atxmega64a4u.insert("USARTC1_BAUDCTRLB", 0x8b7);
        mcu_atxmega64a4u.insert("USARTD0_DATA", 0x9a0);
        mcu_atxmega64a4u.insert("USARTD1_DATA", 0x9b0);
        mcu_atxmega64a4u.insert("RAMPD", 0x38);
        mcu_atxmega64a4u.insert("USARTE0_STATUS", 0xaa1);
        mcu_atxmega64a4u.insert("USARTC1_CTRLC", 0x8b5);
        mcu_atxmega64a4u.insert("USARTD1_BAUDCTRLB", 0x9b7);
        mcu_atxmega64a4u.insert("USARTD0_CTRLC", 0x9a5);
        mcu_atxmega64a4u.insert("USARTD1_BAUDCTRLA", 0x9b6);
        mcu_atxmega64a4u.insert("USARTE0_CTRLA", 0xaa3);
        mcu_atxmega64a4u.insert("PORTA", 0x600);
        mcu_atxmega64a4u.insert("USARTC1_BAUDCTRLA", 0x8b6);
        mcu_atxmega64a4u.insert("RAMPZ", 0x3b);
        mcu_atxmega64a4u.insert("USARTC0_BAUDCTRLB", 0x8a7);
        mcu_atxmega64a4u.insert("USARTD1_CTRLB", 0x9b4);
        mcu_atxmega64a4u.insert("USARTC0_STATUS", 0x8a1);
        mcu_atxmega64a4u.insert("USARTC0_BAUDCTRLA", 0x8a6);
        mcu_atxmega64a4u.insert("USARTD1_STATUS", 0x9b1);
        mcu_atxmega64a4u.insert("USARTC0_DATA", 0x8a0);
        mcu_atxmega64a4u.insert("RAMPY", 0x3a);
        mcu_atxmega64a4u.insert("PORTB", 0x620);
        mcu_atxmega64a4u.insert("USARTC1_CTRLA", 0x8b3);
        mcu_atxmega64a4u.insert("USARTC0_CTRLB", 0x8a4);
        mcu_atxmega64a4u.insert("USARTE0_BAUDCTRLA", 0xaa6);
        mcu_atxmega64a4u.insert("USARTD0_STATUS", 0x9a1);
        mcu_atxmega64a4u.insert("#FLASHEND", 0x10fff);
        mcu_atxmega64a4u.insert("#__AVR_MEGA__", 0x1);
        mcu_atxmega64a4u.insert("#IO_SIZE", 0x1000);
        mcu_atxmega64a4u.insert("#__AVR_XMEGA__", 0x1);
        mcu_atxmega64a4u.insert("#__AVR_2_BYTE_PC__", 0x1);
        mcu_atxmega64a4u.insert("#RAMEND", 0x2fff);
        mcu_atxmega64a4u.insert("#MAPPED_EEPROM_END", 0x17ff);
        mcu_atxmega64a4u.insert("#SPM_PAGESIZE", 0x100);
        mcu_atxmega64a4u.insert("#MAPPED_EEPROM_START", 0x1000);
        mcu_atxmega64a4u.insert("#RAMSTART", 0x2000);
        mcu_atxmega64a4u.insert("#__AVR_ARCH__", 0x68);
        service.mcus.insert("atxmega64a4u", mcu_atxmega64a4u);
        
        let mut mcu_atxmega128a4u: IoRegAddrs = HashMap::new();
        mcu_atxmega128a4u.insert("USARTE0_CTRLB", 0xaa4);
        mcu_atxmega128a4u.insert("SPL", 0x3d);
        mcu_atxmega128a4u.insert("USARTE0_DATA", 0xaa0);
        mcu_atxmega128a4u.insert("PORTD", 0x660);
        mcu_atxmega128a4u.insert("USARTC1_STATUS", 0x8b1);
        mcu_atxmega128a4u.insert("USARTD0_BAUDCTRLB", 0x9a7);
        mcu_atxmega128a4u.insert("SREG", 0x3f);
        mcu_atxmega128a4u.insert("SPH", 0x3e);
        mcu_atxmega128a4u.insert("PORTC", 0x640);
        mcu_atxmega128a4u.insert("USARTC0_CTRLC", 0x8a5);
        mcu_atxmega128a4u.insert("USARTD0_CTRLB", 0x9a4);
        mcu_atxmega128a4u.insert("USARTE0_BAUDCTRLB", 0xaa7);
        mcu_atxmega128a4u.insert("USARTD1_CTRLC", 0x9b5);
        mcu_atxmega128a4u.insert("USARTD0_BAUDCTRLA", 0x9a6);
        mcu_atxmega128a4u.insert("USARTD1_CTRLA", 0x9b3);
        mcu_atxmega128a4u.insert("USARTC1_DATA", 0x8b0);
        mcu_atxmega128a4u.insert("USARTE0_CTRLC", 0xaa5);
        mcu_atxmega128a4u.insert("RAMPX", 0x39);
        mcu_atxmega128a4u.insert("USARTC0_CTRLA", 0x8a3);
        mcu_atxmega128a4u.insert("USARTC1_CTRLB", 0x8b4);
        mcu_atxmega128a4u.insert("USARTD0_CTRLA", 0x9a3);
        mcu_atxmega128a4u.insert("USARTC1_BAUDCTRLB", 0x8b7);
        mcu_atxmega128a4u.insert("USARTD0_DATA", 0x9a0);
        mcu_atxmega128a4u.insert("USARTD1_DATA", 0x9b0);
        mcu_atxmega128a4u.insert("RAMPD", 0x38);
        mcu_atxmega128a4u.insert("USARTE0_STATUS", 0xaa1);
        mcu_atxmega128a4u.insert("USARTC1_CTRLC", 0x8b5);
        mcu_atxmega128a4u.insert("USARTD1_BAUDCTRLB", 0x9b7);
        mcu_atxmega128a4u.insert("USARTD0_CTRLC", 0x9a5);
        mcu_atxmega128a4u.insert("USARTD1_BAUDCTRLA", 0x9b6);
        mcu_atxmega128a4u.insert("USARTE0_CTRLA", 0xaa3);
        mcu_atxmega128a4u.insert("PORTA", 0x600);
        mcu_atxmega128a4u.insert("USARTC1_BAUDCTRLA", 0x8b6);
        mcu_atxmega128a4u.insert("RAMPZ", 0x3b);
        mcu_atxmega128a4u.insert("USARTC0_BAUDCTRLB", 0x8a7);
        mcu_atxmega128a4u.insert("USARTD1_CTRLB", 0x9b4);
        mcu_atxmega128a4u.insert("USARTC0_STATUS", 0x8a1);
        mcu_atxmega128a4u.insert("USARTC0_BAUDCTRLA", 0x8a6);
        mcu_atxmega128a4u.insert("USARTD1_STATUS", 0x9b1);
        mcu_atxmega128a4u.insert("USARTC0_DATA", 0x8a0);
        mcu_atxmega128a4u.insert("RAMPY", 0x3a);
        mcu_atxmega128a4u.insert("PORTB", 0x620);
        mcu_atxmega128a4u.insert("USARTC1_CTRLA", 0x8b3);
        mcu_atxmega128a4u.insert("USARTC0_CTRLB", 0x8a4);
        mcu_atxmega128a4u.insert("USARTE0_BAUDCTRLA", 0xaa6);
        mcu_atxmega128a4u.insert("USARTD0_STATUS", 0x9a1);
        mcu_atxmega128a4u.insert("#FLASHEND", 0x21fff);
        mcu_atxmega128a4u.insert("#__AVR_3_BYTE_PC__", 0x1);
        mcu_atxmega128a4u.insert("#__AVR_MEGA__", 0x1);
        mcu_atxmega128a4u.insert("#IO_SIZE", 0x1000);
        mcu_atxmega128a4u.insert("#__AVR_XMEGA__", 0x1);
        mcu_atxmega128a4u.insert("#RAMEND", 0x3fff);
        mcu_atxmega128a4u.insert("#MAPPED_EEPROM_END", 0x17ff);
        mcu_atxmega128a4u.insert("#SPM_PAGESIZE", 0x100);
        mcu_atxmega128a4u.insert("#MAPPED_EEPROM_START", 0x1000);
        mcu_atxmega128a4u.insert("#RAMSTART", 0x2000);
        mcu_atxmega128a4u.insert("#__AVR_ARCH__", 0x6b);
        service.mcus.insert("atxmega128a4u", mcu_atxmega128a4u);
        

        service
    }

    pub fn get_mcu_registers(&self, mcu: &str) -> Option<&IoRegAddrs> {
        self.mcus.get(mcu)
    }
}
