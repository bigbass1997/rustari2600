use crate::arch::BusAccessable;
use crate::{Bus, Cpu};

#[derive(Clone, Debug, Default)]
pub struct Tia {
    wsync: bool,
    div3_counter: u8,
}
impl Tia {
    /// Perform one clock cycle of the TIA chip. This chip contains a clock divider which
    /// drives the CPU's PHI0 clock input. This function should not be called from within
    /// the CPU.
    /// 
    /// In respect to real time, this function should be called approx 3,584,160 (3.58 MHz) times
    /// per second.
    /// 
    /// The TIA will process its clock first, and then depending on the divider, will clock the CPU.
    pub fn cycle(&mut self, bus: &mut Bus) {
        let mut cpu = &mut bus.cpu;
        
        //TODO: TIA stuff here
        
        
        cpu.rdy = !self.wsync;
        
        self.div3_counter += 1;
        if self.div3_counter == 3 {
            self.div3_counter = 0;
            cpu.cycle(self);
        }
    }
}
impl BusAccessable for Tia {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x00 => unimplemented!(),
            0x01 => unimplemented!(),
            0x02 => self.wsync = true,
            0x03 => unimplemented!(),
            0x04 => unimplemented!(),
            0x05 => unimplemented!(),
            0x06 => unimplemented!(),
            0x07 => unimplemented!(),
            0x08 => unimplemented!(),
            0x09 => unimplemented!(),
            0x0A => unimplemented!(),
            0x0B => unimplemented!(),
            0x0C => unimplemented!(),
            0x0D => unimplemented!(),
            0x0E => unimplemented!(),
            0x0F => unimplemented!(),
            0x10 => unimplemented!(),
            0x11 => unimplemented!(),
            0x12 => unimplemented!(),
            0x13 => unimplemented!(),
            0x14 => unimplemented!(),
            0x15 => unimplemented!(),
            0x16 => unimplemented!(),
            0x17 => unimplemented!(),
            0x18 => unimplemented!(),
            0x19 => unimplemented!(),
            0x1A => unimplemented!(),
            0x1B => unimplemented!(),
            0x1C => unimplemented!(),
            0x1D => unimplemented!(),
            0x1E => unimplemented!(),
            0x1F => unimplemented!(),
            0x20 => unimplemented!(),
            0x21 => unimplemented!(),
            0x22 => unimplemented!(),
            0x23 => unimplemented!(),
            0x24 => unimplemented!(),
            0x25 => unimplemented!(),
            0x26 => unimplemented!(),
            0x27 => unimplemented!(),
            0x28 => unimplemented!(),
            0x29 => unimplemented!(),
            0x2A => unimplemented!(),
            0x2B => unimplemented!(),
            0x2C => unimplemented!(),
            _ => {
                println!("TIA: Invalid write to 0x{:#04X} (0x{:#02X}", addr, data);
            }
        }
    }

    fn read(&self, addr: u16) -> u8 {
        todo!()
    }
}
