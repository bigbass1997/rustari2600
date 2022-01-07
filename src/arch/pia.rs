use crate::arch::BusAccessable;
use crate::{Bus, InfCell};

#[derive(Clone, Debug)]
pub struct Pia {
    ram: [u8; 128],
    intim: u8,
    intim_interval: usize,
    intim_counter: usize,
}
impl Default for Pia {
    fn default() -> Self { Self {
        ram: [0u8; 128],
        intim: 0x20, // Is likely random at cold boot
        intim_interval: 1024, // Stella seems? consistent on this to be 1024
        intim_counter: 0,
    }}
}
impl Pia {
    pub fn cycle(&mut self, bus_cell: &InfCell<Bus>) {
        let bus = bus_cell.get_mut();
        
        self.intim_counter += 1;
        if self.intim_counter == self.intim_interval {
            self.intim_counter = 0;
            if self.intim > 0 {
                self.intim -= 1;
            }
        }
    }
}

impl BusAccessable for Pia {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0080..=0x00FF => self.ram[(addr & 0x007F) as usize] = data,
            _ => panic!("Write attempt to invalid address {:#06X} ({:#04X})", addr, data),
        }
    }
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0080..=0x00FF => self.ram[(addr & 0x007F) as usize],
            0x0280 => 0b11111111, // SWCHA
            0x0281 => unimplemented!(),
            0x0282 => 0b00111111, // SWCHB
            0x0283 => unimplemented!(),
            0x0284 => self.intim, // INTIM
            0x0285 => unimplemented!(), // INSTAT (need to find documentation for this)
            
            0x0294 => unimplemented!(),
            0x0295 => unimplemented!(),
            0x0296 => unimplemented!(),
            0x0297 => unimplemented!(),
            _ => panic!("Read attempt to invalid address {:#06X}", addr),
        }
    }
}