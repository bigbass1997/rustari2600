use std::intrinsics::atomic_nand;
use std::num::Wrapping;
use crate::arch::BusAccessable;
use crate::Bus;
use bitflags::bitflags;



bitflags! {
    pub struct StatusFlags: u8 {
        const Negative          = 0b10000000;
        const Overflow          = 0b01000000;
        const Unused            = 0b00100000;
        const Break             = 0b00010000;
        const Decimal           = 0b00001000;
        const InterruptDisable  = 0b00000100;
        const Zero              = 0b00000010;
        const Carry             = 0b00000001;
    }
}
impl Default for StatusFlags {
    fn default() -> Self {
        StatusFlags::Unused | StatusFlags::Break
    }
}


pub enum AddrMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Zero,
    ZeroX,
    ZeroY,
}


#[derive(Clone, Debug)]
pub struct Cpu {
    pub pc: u16,
    pub sp: Wrapping<u8>,
    pub status: u8,
    pub acc: u8,
    pub x: u8,
    pub y: u8,
    pub rdy: bool,
    prefetch: Option<u8>,
}
impl Default for Cpu {
    fn default() -> Self {
        Self {
            pc: 0xFFFC,
            sp: Wrapping(0), // actually this is potentialy random at power-on // software typically initializes this to 0xFF
            status: StatusFlags::default().bits,
            acc: 0,
            x: 0,
            y: 0,
            rdy: true,
            prefetch: None,
        }
    }
}

impl Cpu {
    pub fn cycle(&mut self, bus: &mut Bus) {
        let opcode = self.prefetch.unwrap_or(self.fetch(bus));
        
        use AddrMode::*;
        match opcode {
            0x00 => self.brk(),
            0x01 => self.ora(IndirectX),
            0x03 => self.slo(IndirectX),
            0x04 => self.nop(Zero),
            0x05 => self.ora(Zero),
            0x06 => self.asl(Zero),
            0x07 => self.slo(Zero),
            0x08 => self.php(Implied),
            0x09 => self.ora(Immediate),
            0x0A => self.asl(Accumulator),
            0x0B => self.anc(),
            0x0C => self.nop(Absolute),
            0x0D => self.ora(Absolute),
            0x0E => self.asl(Absolute),
            0x0F => self.slo(Absolute),
            
            0x10 => self.bpl(Relative),
            0x11 => self.ora(IndirectY),
            0x13 => self.slo(IndirectY),
            0x14 => self.nop(ZeroX),
            0x15 => self.ora(ZeroX),
            0x16 => self.asl(ZeroX),
            0x17 => self.slo(ZeroX),
            0x18 => self.clc(Implied),
            0x19 => self.ora(AbsoluteY),
            0x1A => self.nop(Implied),
            0x1B => self.slo(AbsoluteY),
            0x1C => self.nop(AbsoluteX),
            0x1D => self.ora(AbsoluteX),
            0x1E => self.asl(AbsoluteX),
            0x1F => self.slo(AbsoluteX),
            
            0x20 => self.jsr(),
            0x21 => self.and(IndirectX),
            0x23 => self.rla(IndirectX),
            0x24 => self.bit(Zero),
            0x25 => self.and(Zero),
            0x26 => self.rol(Zero),
            0x27 => self.rla(Zero),
            0x28 => self.plp(Implied),
            0x29 => self.and(Immediate),
            0x2A => self.rol(Accumulator),
            0x2B => self.anc(),
            0x2C => self.bit(Absolute),
            0x2D => self.and(Absolute),
            0x2E => self.rol(Absolute),
            0x2F => self.rla(Absolute),
            
            0x30 => self.bmi(Relative),
            0x31 => self.and(IndirectY),
            0x33 => self.rla(IndirectY),
            0x34 => self.nop(ZeroX),
            0x35 => self.and(ZeroX),
            0x36 => self.rol(ZeroX),
            0x37 => self.rla(ZeroX),
            0x38 => self.sec(Implied),
            0x39 => self.and(AbsoluteY),
            0x3A => self.nop(Implied),
            0x3B => self.rla(AbsoluteY),
            0x3C => self.nop(AbsoluteX),
            0x3D => self.and(AbsoluteX),
            0x3E => self.rol(AbsoluteX),
            0x3F => self.rla(AbsoluteX),
            
            0x40 => self.rti(),
            0x41 => self.eor(IndirectX),
            0x43 => self.sre(IndirectX),
            0x44 => self.nop(Zero),
            0x45 => self.eor(Zero),
            0x46 => self.lsr(Zero),
            0x47 => self.sre(Zero),
            0x48 => self.pha(Implied),
            0x49 => self.eor(Immediate),
            0x4A => self.lsr(Accumulator),
            0x4B => self.asr(),
            0x4C => self.jmp(Absolute),
            0x4D => self.eor(Absolute),
            0x4E => self.lsr(Absolute),
            0x4F => self.sre(Absolute),
            
            0x50 => self.bvc(Relative),
            0x51 => self.eor(IndirectY),
            0x53 => self.sre(IndirectY),
            0x54 => self.nop(ZeroX),
            0x55 => self.eor(ZeroX),
            0x56 => self.lsr(ZeroX),
            0x57 => self.sre(ZeroX),
            0x58 => self.cli(),
            0x59 => self.eor(AbsoluteY),
            0x5A => self.nop(Implied),
            0x5B => self.sre(AbsoluteY),
            0x5C => self.nop(AbsoluteX),
            0x5D => self.eor(AbsoluteX),
            0x5E => self.lsr(AbsoluteX),
            0x5F => self.sre(AbsoluteX),
            
            0x60 => self.rts(Implied),
            0x61 => self.adc(IndirectX),
            0x63 => self.rra(IndirectX),
            0x64 => self.nop(Zero),
            0x65 => self.adc(Zero),
            0x66 => self.ror(Zero),
            0x67 => self.rra(Zero),
            0x68 => self.pla(Implied),
            0x69 => self.adc(Immediate),
            0x6A => self.ror(Accumulator),
            0x6B => self.arr(),
            0x6C => self.jmp(Absolute),
            0x6D => self.adc(Absolute),
            0x6E => self.ror(Absolute),
            0x6F => self.rra(Absolute),
            
            0x70 => self.bvs(Relative),
            0x71 => self.adc(IndirectY),
            0x73 => self.rra(IndirectY),
            0x74 => self.nop(ZeroX),
            0x75 => self.adc(ZeroX),
            0x76 => self.ror(ZeroX),
            0x77 => self.rra(ZeroX),
            0x78 => self.sei(),
            0x79 => self.adc(AbsoluteY),
            0x7A => self.nop(Implied),
            0x7B => self.rra(AbsoluteY),
            0x7C => self.nop(AbsoluteX),
            0x7D => self.adc(AbsoluteX),
            0x7E => self.ror(AbsoluteX),
            0x7F => self.rra(AbsoluteX),
            
            0x80 => self.nop(Immediate),
            0x81 => self.sta(IndirectX),
            0x82 => self.nop(Immediate),
            0x83 => self.sax(IndirectX),
            0x84 => self.sty(Zero),
            0x85 => self.sta(Zero),
            0x86 => self.stx(Zero),
            0x87 => self.sax(Zero),
            0x88 => self.dey(Implied),
            0x89 => self.nop(Immediate),
            0x8A => self.txa(Implied),
            0x8B => self.ane(),
            0x8C => self.sty(Absolute),
            0x8D => self.sta(Absolute),
            0x8E => self.stx(Absolute),
            0x8F => self.sax(Absolute),
            
            0x90 => self.bcc(Relative),
            0x91 => self.sta(IndirectY),
            0x93 => self.sha(IndirectY),
            0x94 => self.sty(ZeroX),
            0x95 => self.sta(ZeroX),
            0x96 => self.stx(ZeroY),
            0x97 => self.sax(ZeroY),
            0x98 => self.tya(Implied),
            0x99 => self.sta(AbsoluteY),
            0x9A => self.txs(Implied),
            0x9B => self.shs(),
            0x9C => self.shy(),
            0x9D => self.sta(AbsoluteX),
            0x9E => self.shx(),
            0x9F => self.sha(AbsoluteY),
            
            0xA0 => self.ldy(Immediate),
            0xA1 => self.lda(IndirectX),
            0xA2 => self.ldx(Immediate),
            0xA3 => self.lax(IndirectX),
            0xA4 => self.ldy(Zero),
            0xA5 => self.lda(Zero),
            0xA6 => self.ldx(Zero),
            0xA7 => self.lax(Zero),
            0xA8 => self.tay(Implied),
            0xA9 => self.lda(Immediate),
            0xAA => self.tax(Implied),
            0xAB => self.lxa(),
            0xAC => self.ldy(Absolute),
            0xAD => self.lda(Absolute),
            0xAE => self.ldx(Absolute),
            0xAF => self.lax(Absolute),
            
            0xB0 => self.bcs(Relative),
            0xB1 => self.lda(IndirectY),
            0xB3 => self.lax(IndirectY),
            0xB4 => self.ldy(ZeroX),
            0xB5 => self.lda(ZeroX),
            0xB6 => self.ldx(ZeroY),
            0xB7 => self.lax(ZeroY),
            0xB8 => self.clv(Implied),
            0xB9 => self.lda(AbsoluteY),
            0xBA => self.tsx(Implied),
            0xBB => self.las(),
            0xBC => self.ldy(AbsoluteX),
            0xBD => self.lda(AbsoluteX),
            0xBE => self.ldx(AbsoluteY),
            0xBF => self.lax(AbsoluteY),
            
            0xC0 => self.cpy(Immediate),
            0xC1 => self.cmp(IndirectX),
            0xC2 => self.nop(Immediate),
            0xC3 => self.dcp(IndirectX),
            0xC4 => self.cpy(Zero),
            0xC5 => self.cmp(Zero),
            0xC6 => self.dec(Zero),
            0xC7 => self.dcp(Zero),
            0xC8 => self.iny(Implied),
            0xC9 => self.cmp(Immediate),
            0xCA => self.dex(Implied),
            0xCB => self.sbx(),
            0xCC => self.cpy(Absolute),
            0xCD => self.cmp(Absolute),
            0xCE => self.dec(Absolute),
            0xCF => self.dcp(Absolute),
            
            0xD0 => self.bne(Relative),
            0xD1 => self.cmp(IndirectY),
            0xD3 => self.dcp(IndirectY),
            0xD4 => self.nop(ZeroX),
            0xD5 => self.cmp(ZeroX),
            0xD6 => self.dec(ZeroX),
            0xD7 => self.dcp(ZeroX),
            0xD8 => self.cld(),
            0xD9 => self.cmp(AbsoluteY),
            0xDA => self.nop(Implied),
            0xDB => self.dcp(AbsoluteY),
            0xDC => self.nop(AbsoluteX),
            0xDD => self.cmp(AbsoluteX),
            0xDE => self.dec(AbsoluteX),
            0xDF => self.dcp(AbsoluteX),
            
            0xE0 => self.cpx(Immediate),
            0xE1 => self.sbc(IndirectX),
            0xE2 => self.nop(Immediate),
            0xE3 => self.isb(IndirectX),
            0xE4 => self.cpx(Zero),
            0xE5 => self.sbc(Zero),
            0xE6 => self.inc(Zero),
            0xE7 => self.isb(Zero),
            0xE8 => self.inx(Implied),
            0xE9 => self.sbc(Immediate),
            0xEA => self.nop(Implied),
            0xEB => self.sbc(Immediate),
            0xEC => self.cpx(Absolute),
            0xED => self.sbc(Absolute),
            0xEE => self.inc(Absolute),
            0xEF => self.isb(Absolute),
            
            0xF0 => self.beq(Relative),
            0xF1 => self.sbc(IndirectY),
            0xF3 => self.isb(IndirectY),
            0xF4 => self.nop(ZeroX),
            0xF5 => self.sbc(ZeroX),
            0xF6 => self.inc(ZeroX),
            0xF7 => self.isb(ZeroX),
            0xF8 => self.sed(),
            0xF9 => self.sbc(AbsoluteY),
            0xFA => self.nop(Implied),
            0xFB => self.isb(AbsoluteY),
            0xFC => self.nop(AbsoluteX),
            0xFD => self.sbc(AbsoluteX),
            0xFE => self.inc(AbsoluteX),
            0xFF => self.isb(AbsoluteX),
            
            _ => panic!("Attempt to run invalid opcode! PC: {:#06X}, Op: {:#06X}", self.pc, opcode)
        }
    }
    
    fn adc(&mut self, mode: AddrMode) {}
    fn anc(&mut self) {}
    fn and(&mut self, mode: AddrMode) {}
    fn ane(&mut self) {}
    fn arr(&mut self) {}
    fn asl(&mut self, mode: AddrMode) {}
    fn asr(&mut self) {}
    fn bcc(&mut self, mode: AddrMode) {}
    fn bcs(&mut self, mode: AddrMode) {}
    fn beq(&mut self, mode: AddrMode) {}
    fn bit(&mut self, mode: AddrMode) {}
    fn bmi(&mut self, mode: AddrMode) {}
    fn bne(&mut self, mode: AddrMode) {}
    fn bpl(&mut self, mode: AddrMode) {}
    fn brk(&mut self) {}
    fn bvc(&mut self, mode: AddrMode) {}
    fn bvs(&mut self, mode: AddrMode) {}
    fn clc(&mut self, mode: AddrMode) {}
    fn cld(&mut self) {}
    fn cli(&mut self) {}
    fn clv(&mut self, mode: AddrMode) {}
    fn cmp(&mut self, mode: AddrMode) {}
    fn cpx(&mut self, mode: AddrMode) {}
    fn cpy(&mut self, mode: AddrMode) {}
    fn dcp(&mut self, mode: AddrMode) {}
    fn dec(&mut self, mode: AddrMode) {}
    fn dex(&mut self, mode: AddrMode) {}
    fn dey(&mut self, mode: AddrMode) {}
    fn eor(&mut self, mode: AddrMode) {}
    fn inc(&mut self, mode: AddrMode) {}
    fn inx(&mut self, mode: AddrMode) {}
    fn iny(&mut self, mode: AddrMode) {}
    fn isb(&mut self, mode: AddrMode) {}
    fn jmp(&mut self, mode: AddrMode) {}
    fn jsr(&mut self) {}
    fn las(&mut self) {}
    fn lax(&mut self, mode: AddrMode) {}
    fn lda(&mut self, mode: AddrMode) {}
    fn ldx(&mut self, mode: AddrMode) {}
    fn ldy(&mut self, mode: AddrMode) {}
    fn lsr(&mut self, mode: AddrMode) {}
    fn lxa(&mut self) {}
    fn nop(&mut self, mode: AddrMode) {}
    fn ora(&mut self, mode: AddrMode) {}
    fn pha(&mut self, mode: AddrMode) {}
    fn php(&mut self, mode: AddrMode) {}
    fn pla(&mut self, mode: AddrMode) {}
    fn plp(&mut self, mode: AddrMode) {}
    fn rla(&mut self, mode: AddrMode) {}
    fn rra(&mut self, mode: AddrMode) {}
    fn rol(&mut self, mode: AddrMode) {}
    fn ror(&mut self, mode: AddrMode) {}
    fn rti(&mut self) {}
    fn rts(&mut self, mode: AddrMode) {}
    fn sax(&mut self, mode: AddrMode) {}
    fn sbc(&mut self, mode: AddrMode) {}
    fn sbx(&mut self) {}
    fn sec(&mut self, mode: AddrMode) {}
    fn sed(&mut self) {}
    fn sei(&mut self) {}
    fn sha(&mut self, mode: AddrMode) {}
    fn shs(&mut self) {}
    fn shx(&mut self) {}
    fn shy(&mut self) {}
    fn slo(&mut self, mode: AddrMode) {}
    fn sre(&mut self, mode: AddrMode) {}
    fn sta(&mut self, mode: AddrMode) {}
    fn stx(&mut self, mode: AddrMode) {}
    fn sty(&mut self, mode: AddrMode) {}
    fn tax(&mut self, mode: AddrMode) {}
    fn tay(&mut self, mode: AddrMode) {}
    fn tsx(&mut self, mode: AddrMode) {}
    fn txa(&mut self, mode: AddrMode) {}
    fn txs(&mut self, mode: AddrMode) {}
    fn tya(&mut self, mode: AddrMode) {}
    
    fn fetch(&mut self, bus: &mut Bus) -> u8 {
        let fetch = bus.read(self.pc);
        self.pc += 1;
        
        fetch
    }
}

impl BusAccessable for Cpu {
    fn write(&mut self, addr: u16, data: u8) {
        todo!()
    }

    fn read(&self, addr: u16) -> u8 {
        todo!()
    }
}