use std::path::PathBuf;
use std::time::{Duration, Instant};
use clap::{App, AppSettings, Arg};
use crate::arch::Bus;
use crate::arch::cpu::Cpu;
use crate::util::InfCell;

mod arch;
mod util;

fn main() {
    let matches = App::new("Rustari2600")
        .arg(Arg::new("rom")
            .required(true)
            .takes_value(true))
        .setting(AppSettings::NextLineHelp)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .get_matches();
    
    let bus_cell = InfCell::new(Bus::default());
    let bus = bus_cell.get_mut();
    let bus_ref = bus_cell.get_mut();
    
    bus.cart.set_rom(&std::fs::read(PathBuf::from(matches.value_of("rom").unwrap())).unwrap());
    bus.cpu.init_pc(bus_ref);
    
    loop {
        let start = Instant::now();
        for _ in 0..3584160 {
            bus.tia.cycle(&bus_cell);
        }
        
        let elapsed = start.elapsed();
        if elapsed.as_micros() < 1000000 {
            std::thread::sleep(Duration::from_micros(999940 - elapsed.as_micros() as u64))
        }
        println!("time to simulate 1 second: {:.6}sec ({}us)", start.elapsed().as_secs_f64(), elapsed.as_micros());
    }
}
