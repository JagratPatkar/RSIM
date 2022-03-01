extern crate bit;
use std::io::Result;
use crate::im::InsMem;
use crate::cpu::CPU;
mod decoder;
mod dm;
mod rfm;
mod alu;
mod im;
mod cpu;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).expect("no path given");
    println!("RSIM Configured!");
    let mut ins_mem = InsMem{
        mem : Vec::new(),
        path : std::path::PathBuf::from(path)
    };
    println!("Reading the Instructions!");
    ins_mem.populate_mem()?;
    let mut cpu = CPU{
        counter : 0x0000,
        reset_val : 0x0000
    };
    cpu.start_fetch(&mut ins_mem);
    println!("Simulation Complete!");
    Ok(())
}