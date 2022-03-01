extern crate bit;
use std::fs::File;
use std::io::{BufReader,Read};
use std::io::Result;
use bit::BitIndex;
use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;
use crate::alu::ALU;
use crate::im::InsMem;
use crate::pc::PC;
mod decoder;
mod dm;
mod rfm;
mod alu;
mod im;
mod pc;

fn main() -> Result<()> {
    let path = std::env::args().nth(1).expect("no path given");
    println!("RSIM Configured!");
    let mut ins_mem = InsMem{
        mem : Vec::new(),
        path : std::path::PathBuf::from(path)
    };
    println!("Reading the Instructions!");
    ins_mem.populate_mem()?;
    let mut pc = PC{
        counter : 0x0000,
        reset_val : 0x0000
    };
    pc.start_fetch(&mut ins_mem);
    println!("Simulation Complete!");
    Ok(())
}