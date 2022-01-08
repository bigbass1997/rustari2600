use crate::arch::BusAccessable;
use crate::{Bus, Cpu, InfCell};

pub const NTSC_COLOR_LUT: [u32; 128] = [
    0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC,//
    0x444400, 0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68,//
    
    0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC,
    0x444400, 0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68,
    0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC,
    
    0x78005C, 0x8C2074, 0xA03C88, 0xB0589C, 0xC070B0, 0xD084C0, 0xDC9CD0, 0xECB0E0,//
    0x480078, 0x602090, 0x783CA4, 0x8C58B8, 0xA070CC, 0xB484DC, 0xC49CEC, 0xD4B0FC,//
    
    0x444400, 0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68,
    0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC,
    0x444400, 0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68,
    0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC,
    0x444400, 0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68,
    0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC,
    0x444400, 0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68,
    0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC,
    0x444400, 0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68,
];

pub const SECAM_COLOR_LUT: [u32; 8] = [
    0x000000, 0x2121FF, 0xF03C79, 0xFF50FF, 0x7FFF00, 0x7FFFFF, 0xFFFF3F, 0xFFFFFF
];

#[derive(Copy, Clone, Debug, Default)]
pub struct CycleCounter {
    pub(crate) osc: usize,
    pub(crate) div3: u8,
    pub(crate) scanline: usize,
    pub(crate) color_clock: usize,
    pub(crate) frame_cpu_counter: usize,
    pub(crate) frame_counter: usize,
}
impl CycleCounter {
    fn osc_cycle(&mut self) {
        self.osc += 1;
        self.div3 += 1;
        if self.div3 == 3 {
            self.div3 = 0;
        }
        
        self.color_clock += 1;
        if self.color_clock == 228 {
            self.scanline += 1;
            self.color_clock = 0;
            
            if self.scanline == 262 {
                self.scanline = 0;
                //self.frame_counter += 1;
            }
        }
    }
    
    fn pixel_index(&self) -> usize {
        (self.scanline * 228) + self.color_clock
    }
}

#[derive(Clone, Debug)]
pub struct Tia {
    vsync: bool,
    vsync_trigger: bool,
    vblank: bool,
    wsync: bool,
    
    colupf: u8,
    colubk: u8,
    
    ctrlpf: u8,
    
    pf0: u8,
    pf1: u8,
    pf2: u8,
    
    pub cycles: CycleCounter,
    pub framebuffer: [u32; 228 * 262],
    pub fb_color: u32,
}
impl Default for Tia {
    fn default() -> Self { Self {
        vsync: false,
        vsync_trigger: false,
        vblank: false,
        wsync: false,
        
        colupf: 0,
        colubk: 0,
        
        ctrlpf: 0,
        
        pf0: 0,
        pf1: 0,
        pf2: 0,
        
        cycles: Default::default(),
        framebuffer: [0u32; 228 * 262],
        fb_color: 0,
    }}
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
    pub fn cycle(&mut self, bus_cell: &InfCell<Bus>) {
        let bus = bus_cell.get_mut();
        let bus_ref = bus_cell.get_mut();
        let mut cpu = &mut bus.cpu;
        let mut pia = &mut bus.pia;
        
        // === OSC CLOCK === //
        //TODO: TIA stuff here
        if !self.vblank && self.cycles.color_clock >= 68 && bus.tia.cycles.frame_counter > 0 {
            let pixel = self.cycles.color_clock - 68;
            let dot = pixel / 4;
            let pf_bit = self.pf_lut(dot, self.ctrlpf & 0b1 != 0);
            let color = self.color_lut(pf_bit);
            //println!("scan: {}, pixel: {}, dot: {}, pf_bit: {}, color: {:08X}", self.cycles.scanline, pixel, dot, pf_bit, color);
            
            self.framebuffer[self.cycles.pixel_index()] = color;
            
            //self.framebuffer[self.cycles.pixel_index()] = ((self.pf0 as u32) << 16) | ((self.pf1 as u32) << 8) | (self.pf2 as u32);
        }
        
        cpu.rdy = !self.wsync;
        
        if self.cycles.div3 == 0 {
            // === Phi 0 CLOCK === //
            //println!("Cycles: {}", self.cycles.frame_cpu_counter);
            self.cycles.frame_cpu_counter += 1;
            
            cpu.cycle(bus_cell);
            
            // === Phi 2 CLOCK === //
            pia.cycle(bus_cell);
        }
        
        
        //println!("FRAME: {}, SCANLINE: {}, HORIZ: {}, INTIM: {:02X}, INTIM_COUNTER: {:04X}, INTERVAL: {} ({})", self.cycles.frame_counter, self.cycles.scanline, self.cycles.color_clock, bus.pia.intim, bus.pia.intim_counter, bus.pia.intim_interval, bus.pia.intim_interval_active);
        self.cycles.osc_cycle();
        if self.cycles.color_clock == 0 {
            self.wsync = false;
        }
        if self.vsync_trigger && !self.vsync {
            self.cycles.frame_cpu_counter = 0;
            self.cycles.scanline = 0;
            self.cycles.frame_counter += 1;
            //println!("=================================================================");
            //println!("======================= NEW FRAME STARTED =======================");
            //println!("=================================================================");
            self.vsync_trigger = false;
        }
    }
    
    
    fn pf_lut(&self, dot_index: usize, r: bool) -> u8 {
        const B7: u8 = 0b10000000;
        const B6: u8 = 0b01000000;
        const B5: u8 = 0b00100000;
        const B4: u8 = 0b00010000;
        const B3: u8 = 0b00001000;
        const B2: u8 = 0b00000100;
        const B1: u8 = 0b00000010;
        const B0: u8 = 0b00000001;
        
        match dot_index {
            0  => self.pf0 & B4,
            1  => self.pf0 & B5,
            2  => self.pf0 & B6,
            3  => self.pf0 & B7,
            
            4  => self.pf1 & B7,
            5  => self.pf1 & B6,
            6  => self.pf1 & B5,
            7  => self.pf1 & B4,
            8  => self.pf1 & B3,
            9  => self.pf1 & B2,
            10 => self.pf1 & B1,
            11 => self.pf1 & B0,
            
            12 => self.pf2 & B0,
            13 => self.pf2 & B1,
            14 => self.pf2 & B2,
            15 => self.pf2 & B3,
            16 => self.pf2 & B4,
            17 => self.pf2 & B5,
            18 => self.pf2 & B6,
            19 => self.pf2 & B7,
            
            20..=39 if r => self.pf_lut((-((dot_index - 19) as isize) + 20) as usize, false),
            20..=39 => self.pf_lut(dot_index - 20, false),
            
            _ => panic!("invalid dot index: {}", dot_index)
        }
    }
    
    fn color_lut(&self, reg: u8) -> u32 {
        let colu = if reg != 0 { self.colupf } else { self.colubk };
        
        /*if reg != 0 {
            0x00ABABAB
        } else {
            0
        }*/
        
        NTSC_COLOR_LUT[(colu / 2) as usize]
    }
}
impl BusAccessable for Tia {
    fn write(&mut self, addr: u16, data: u8) {
        //println!("TIA Write: {:02X} to {:04X}", data, addr);
        match addr {
            0x00 => {
                let past = self.vsync;
                self.vsync = (data & 0b00000010) != 0;
                if !past && self.vsync {
                    self.vsync_trigger = true;
                }
            },
            0x01 => self.vblank = (data & 0b11000010) != 0,
            0x02 => self.wsync = true,
            0x03 => unimplemented!(),
           /* 0x04 => unimplemented!(),
            0x05 => unimplemented!(),
            0x06 => unimplemented!(),
            0x07 => unimplemented!(),*/
            0x08 => self.colupf = data & 0b11111110,
            0x09 => self.colubk = data & 0b11111110,
            0x0A => self.ctrlpf = data & 0b00110111,
            /*0x0B => unimplemented!(),
            0x0C => unimplemented!(),*/
            0x0D => self.pf0 = data & 0b11110000,
            0x0E => self.pf1 = data,
            0x0F => self.pf2 = data,
            /*0x10 => unimplemented!(),
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
            0x2C => (), //TODO*/
            _ => {
                //println!("TIA: Invalid write to {:04X} ({:02X})", addr, data);
            }
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        //println!("TIA Read from {:04X}", addr);
        match addr {
            0x30 => unimplemented!(), // CXM0P
            0x31 => unimplemented!(), // CXM1P
            0x32 => 0b00000000, // CXP0FB
            0x33 => 0b00000000, // CXP1FB
            0x34 => 0b00000000, // CXM0FB
            0x35 => 0b00000000, // CXM1FB
            0x36 => 0b00000000, // CXBLPF
            0x37 => 0b00000000, // CXPPMM
            0x38 => unimplemented!(),
            0x39 => unimplemented!(),
            0x3A => unimplemented!(),
            0x3B => unimplemented!(),
            0x3C => 0b10000000, // INPT4 //TODO: Besides normal input handling, it appears this register has other functionality
            0x3D => 0b10000000, // INPT5 //TODO: Besides normal input handling, it appears this register has other functionality
            _ => 0//panic!("TIA: Invalid read from {:04X}", addr)
        }
    }
}