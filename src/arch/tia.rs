use crate::arch::BusAccessable;
use crate::{Bus, Cpu, InfCell};

#[derive(Copy, Clone, Debug, Default)]
pub struct CycleCounter {
    pub(crate) osc: usize,
    pub(crate) div3: u8,
    pub(crate) scanline: usize,
    pub(crate) color_clock: usize,
    pub(crate) debug_cpu_counter: usize,
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
    wsync: bool,
    pub cycles: CycleCounter,
    pub framebuffer: [u32; 228 * 262],
    pub fb_color: u32,
}
impl Default for Tia {
    fn default() -> Self { Self {
        vsync: false,
        wsync: false,
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
        
        // === OSC CLOCK === //
        //TODO: TIA stuff here
        /*let pixel = &mut self.framebuffer[self.cycles.pixel_index()];
        if self.cycles.scanline < 40 {
            *pixel = 0x00767676;
        } else if self.cycles.scanline >= 232 {
            *pixel = 0x009A9A9A;
        } else if self.cycles.color_clock >= 68 && self.cycles.scanline >= 40 && self.cycles.scanline < 232 {
            *pixel = self.fb_color;
            
            self.fb_color += 1000;
            if self.fb_color > 0x00FFFFFF {
                self.fb_color = 0;
            }
        }*/
        
        
        cpu.rdy = !self.wsync;
        
        if self.cycles.div3 == 0 {
            // === Phi 0 CLOCK === //
            println!("Cycles: {}", self.cycles.debug_cpu_counter);
            self.cycles.debug_cpu_counter += 1;
            
            cpu.cycle(bus_cell);
            
            // === Phi 2 CLOCK === //
            //TODO
        }
        
        
        println!("SCANLINE: {}, HORIZ: {}", self.cycles.scanline, self.cycles.color_clock);
        self.cycles.osc_cycle();
        if self.cycles.color_clock == 0 {
            self.wsync = false;
        }
    }
}
impl BusAccessable for Tia {
    fn write(&mut self, addr: u16, data: u8) {
        println!("TIA Write: {:02X} to {:04X}", data, addr);
        match addr {
            0x00 => self.vsync = (data & 0b10) != 0,
            0x01 => unimplemented!(),
            0x02 => self.wsync = true,
            0x03 => unimplemented!(),
           /* 0x04 => unimplemented!(),
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
            0x2C => (), //TODO*/
            _ => {
                println!("TIA: Invalid write to {:#04X} ({:#02X})", addr, data);
            }
        }
    }

    fn read(&self, addr: u16) -> u8 {
        todo!()
    }
}
