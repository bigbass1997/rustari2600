use std::path::PathBuf;
use std::time::{Duration, Instant};
use clap::{App, AppSettings, Arg};
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};
use crate::arch::Bus;
use crate::arch::cpu::Cpu;
use crate::util::InfCell;

mod arch;
mod util;

const DEBUG_UPDATE_PER_PIXEL: bool = false;
const DEBUG_UPDATE_PER_FRAME: bool = true;

fn main() {
    let matches = App::new("Rustari2600")
        .arg(Arg::new("rom")
            .required(true)
            .takes_value(true))
        .setting(AppSettings::NextLineHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();
    
    let mut window = Window::new("Rustari2600", 228 * 3 / 2, 262, WindowOptions {
        borderless: false,
        title: true,
        resize: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        transparency: false,
        none: false
    }).unwrap();
    
    let bus_cell = InfCell::new(Bus::default());
    let bus = bus_cell.get_mut();
    let bus_ref = bus_cell.get_mut();
    
    bus.cart.set_rom(&std::fs::read(PathBuf::from(matches.value_of("rom").unwrap())).unwrap());
    bus.cpu.init_pc(bus_ref);
    
    loop {
        let start = Instant::now();
        for _ in 0..(3584160/60) {
            bus.tia.cycle(&bus_cell);
            
            if update_window(bus, &mut window) {
                return;
            }
        }
        
        let elapsed = start.elapsed();
        if elapsed.as_micros() < 1000000/60 {
            std::thread::sleep(Duration::from_micros((999940 - elapsed.as_micros() as u64)/60))
        }
        println!("time to simulate 1/60 second: {:.6}sec ({}us)", start.elapsed().as_secs_f64(), elapsed.as_micros());
    }
}

fn update_window(bus: &mut Bus, window: &mut Window) -> bool {
    if bus.tia.cycles.color_clock == 0 /*&& bus.tia.cycles.scanline == 0*/ {
        window.update_with_buffer(&bus.tia.framebuffer, 228, 262).unwrap();
    }
    
    if window.is_key_down(Key::Escape) || !window.is_open() {
        return true;
    }
    
    false
}