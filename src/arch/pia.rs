use crate::arch::BusAccessable;
use crate::{Bus, InfCell};

#[derive(Clone, Debug)]
pub struct Pia {
    ram: [u8; 128],
    pub(crate) intim: u8,
    pub(crate) intim_interval: usize,
    pub(crate) intim_interval_active: bool,
    pub(crate) intim_counter: usize,
    pub(crate) intim_trigger: bool,
}
impl Default for Pia {
    fn default() -> Self { Self {
        ram: [0u8; 128],
        intim: 0x0A, // Is likely random at cold boot
        intim_interval: 1024, // Stella seems? consistent on this to be 1024
        intim_interval_active: true,
        intim_counter: 1,
        intim_trigger: false,
    }}
}
impl Pia {
    pub fn cycle(&mut self, bus_cell: &InfCell<Bus>) {
        let bus = bus_cell.get_mut();
        
        
        if self.intim_trigger {
            self.intim_counter = 1;
            self.intim_interval_active = true;
            self.intim_trigger = false;
        } else {
            if self.intim_interval_active {
                self.intim_counter -= 1;
                if self.intim_counter == 0 {
                    self.intim = self.intim.wrapping_sub(1);
                    if self.intim == 0xFF { // underflow occured
                        self.intim_interval_active = false;
                    }
                    
                    if self.intim_interval_active {
                        self.intim_counter = self.intim_interval;
                    } else {
                        self.intim_counter = 1;
                    }
                }
            } else {
                self.intim = self.intim.wrapping_sub(1);
            }
        }
    }
    
    fn setup_intim(&mut self, intim: u8, interval: usize) {
        self.intim = intim;
        self.intim_interval = interval;
        self.intim_trigger = true;
    }
}

impl BusAccessable for Pia {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0080..=0x00FF => self.ram[(addr & 0x007F) as usize] = data,
            
            0x0294 => self.setup_intim(data, 1),
            0x0295 => self.setup_intim(data, 8),
            0x0296 => self.setup_intim(data, 64),
            0x0297 => self.setup_intim(data, 1024),
            _ => panic!("Write attempt to invalid address {:#06X} ({:#04X})", addr, data),
        }
    }
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0080..=0x00FF => self.ram[(addr & 0x007F) as usize],
            0x0280 => 0b11111111, // SWCHA
            0x0281 => unimplemented!(),
            0x0282 => 0b00111111, // SWCHB
            0x0283 => unimplemented!(),
            0x0284 => {
                self.intim_interval_active = true;
                self.intim
            }, // INTIM
            0x0285 => unimplemented!(), // INSTAT (need to find documentation for this)
            
            0x0294 => unimplemented!(),
            0x0295 => unimplemented!(),
            0x0296 => unimplemented!(),
            0x0297 => unimplemented!(),
            _ => panic!("Read attempt to invalid address {:#06X}", addr),
        }
    }
}