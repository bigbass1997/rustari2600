use crate::arch::BusAccessable;

#[derive(Clone, Debug)]
pub struct Cartridge {
    rom: Vec<u8>,
}
impl Default for Cartridge {
    fn default() -> Self {
        Self {
            rom: vec![0; 1024 * 4]
        }
    }
}

impl BusAccessable for Cartridge {
    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }
    fn read(&mut self, addr: u16) -> u8 {
        self.rom[(addr & 0x0FFF) as usize]
    }
}

impl Cartridge {
    pub fn set_rom(&mut self, rom: &Vec<u8>) {
        self.rom = rom.to_owned();
        if self.rom.len() < 4096 {
            self.rom.resize(4096, 0);
        }
    }
}