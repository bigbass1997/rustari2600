use std::time::{Duration, Instant};
use crate::arch::Bus;
use crate::arch::cpu::Cpu;
use crate::arch::tia::Tia;
use crate::util::InfCell;

mod arch;
mod util;

fn main() {
    let bus_cell = InfCell::new(Bus::default());
    let bus = bus_cell.get_mut();
    let bus_ref = bus_cell.get_mut();
    
    loop {
        let start = Instant::now();
        for _ in 0..3584160 {
            bus.tia.cycle(bus_ref);
        }
        
        let elapsed = start.elapsed();
        if elapsed.as_micros() < 1000000 {
            std::thread::sleep(Duration::from_micros(999940 - elapsed.as_micros() as u64))
        }
        println!("time to simulate 1 second: {:.6}sec ({}us)", start.elapsed().as_secs_f64(), elapsed.as_micros());
    }
}
