use crate::arch::BusAccessable;

#[derive(Clone, Debug)]
pub struct Pia {
    ram: [u8; 128],
}
impl Default for Pia {
    fn default() -> Self { Self {
        ram: [0u8; 128],
    }}
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
            _ => panic!("Read attempt to invalid address {:#06X}", addr),
        }
    }
}