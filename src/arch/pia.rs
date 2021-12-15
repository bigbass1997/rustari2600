use crate::arch::BusAccessable;

#[derive(Clone, Debug, Default)]
pub struct Pia {
    
}

impl BusAccessable for Pia {
    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
    fn read(&self, addr: u16) -> u8 {
        todo!()
    }
}