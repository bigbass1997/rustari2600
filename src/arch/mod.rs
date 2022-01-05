use crate::arch::cartridge::Cartridge;
use crate::arch::cpu::Cpu;
use crate::arch::pia::Pia;
use crate::arch::tia::Tia;

pub mod tia;
pub mod cpu;
pub mod pia;
pub mod cartridge;

pub trait BusAccessable {
    fn write(&mut self, addr: u16, data: u8);
    fn read(&self, addr: u16) -> u8;
}

#[derive(Clone, Default, Debug)]
pub struct Bus {
    pub tia: Tia,
    pub cpu: Cpu,
    pub pia: Pia,
    pub cart: Cartridge,
}

impl BusAccessable for Bus {
    fn write(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000..=0x002C => self.tia.write(addr, data),
            0x0080..=0x00FF | 0x0280..=0x0297 => self.pia.write(addr, data),
            0x0100..=0x01FF => self.cpu.write(addr, data),
            0xF000..=0xFFFF => self.cart.write(addr, data),
            _ => panic!("Write attempt to invalid address {:#06X} ({:#04X})", addr, data),
        }
    }

    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x000D | 0x0030..=0x003D => self.tia.read(addr),
            0x0080..=0x00FF | 0x0280..=0x0297 => self.pia.read(addr),
            0x0100..=0x01FF => self.cpu.read(addr),
            0xF000..=0xFFFF => self.cart.read(addr),
            _ => panic!("Read attempt to invalid address {:#06X}", addr),
        }
    }
}