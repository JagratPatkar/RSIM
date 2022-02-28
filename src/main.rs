extern crate bit;
use std::fs::File;
use std::io::{BufReader,Read};
use std::io::Result;
use bit::BitIndex;

struct InsMem{
    mem : Vec<u8>,
    path : String
}

impl InsMem{
    fn populate_mem(&mut self) -> Result<()> {
        let f = File::open(self.path.as_str())?;
        let reader = BufReader::new(f);
        let mut b_reader = reader.bytes();
        loop{
            let byte = b_reader.next();
            if let Some(c) = byte.as_ref(){
                if let Ok(b) = c { self.mem.push(*b); }
                else { break; } 
            } 
            else { break; } 
        }
        Ok(())
    }

    fn get_ins(&mut self,addr: u32) -> Option<u32> {
        let mut va = addr as usize;
        let mut b : [u8;4] = [0,0,0,0];
        let mut i = 0;
        loop{
            if i < 4 {
                if let Some(val) = self.mem.get(va){
                    b[i] = *val;
                    i += 1;
                    let p = va as u32;
                    va = (p + 0x01) as usize;
                }
                else{ return None; }
            }else { break; }
        }
        Some(u32::from_be_bytes(b))
    }

    fn print_mem(&self){
        println!("{:?}",self.mem);
    }
}


struct PC{
    counter : u32,
    reset_val : u32
}

impl PC{
    fn start_fetch(&mut self,mem : &mut InsMem){
        loop{
            if let Some(x) = mem.get_ins(self.counter) {
                let inst = x;
                self.counter = self.counter + 0x04;
                // Send to decoder
                let mut decoder = Decoder{
                    inst,
                    imm : 0x0
                };
                decoder.init_imm();
                println!("{}",decoder.imm)
            } else{ break; }
        }
    }
}

struct Decoder{
    inst : u32,
    imm : u32
}

impl Decoder{
    fn is_u(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 5u32) | (val == 13u32) { return true; } 
        false
    }

    fn is_j(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if val == 27u32 { return true; } 
        false
    }

    fn is_b(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if val == 24u32 { return true; } 
        false
    }

    fn is_s(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 8u32) | (val == 9u32) { return true; } 
        false
    }

    fn is_r(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 11u32) | (val == 12u32) 
           | (val == 14u32) | (val == 20u32) 
        { return true; } 
        false
    }

    fn is_i(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 0u32) | (val == 1u32) 
           | (val == 4u32) | (val == 6u32) 
           | (val == 25u32) 
        { return true; } 
        false
    }

    fn rs1(&mut self) -> u32 
    { self.inst.bit_range(20..25) }

    fn rs2(&mut self) -> u32 
    { self.inst.bit_range(15..20) }

    fn rd(&mut self) -> u32
    { self.inst.bit_range(15..20) }

    fn rs1_valid(&mut self) -> bool
    { self.is_r() | self.is_s() | self.is_b() | self.is_i() }

    fn rs2_valid(&mut self) -> bool
    { self.is_r() | self.is_s() | self.is_b() }

    fn rd_valid(&mut self) -> bool
    { (self.is_r() | self.is_i() | self.is_j() | self.is_u()) & (self.rd() != 0)  }

    fn imm_valid(&mut self) -> bool 
    { self.is_s() | self.is_i() | self.is_j() | self.is_u() | self.is_b()  }

    fn imm_i(&mut self) {
        self.imm.set_bit_range(0..10,self.inst.bit_range(20..30));
        if self.inst.bit(31) {  self.imm.set_bit_range(11..32,0b11111111111111111111); }
        else { self.imm.set_bit_range(11..32,0b00000000000000000000); }
    }

    fn imm_s(&mut self) {
        self.imm.set_bit_range(0..5,self.inst.bit_range(7..11));
        self.imm.set_bit_range(5..11,self.inst.bit_range(25..30));
        if self.inst.bit(31) {  self.imm.set_bit_range(11..32,0b11111111111111111111); }
        else { self.imm.set_bit_range(11..32,0b00000000000000000000); }
    }

    fn imm_b(&mut self) {
        self.imm.set_bit(0,false);
        self.imm.set_bit_range(1..5,self.inst.bit_range(8..11));
        self.imm.set_bit_range(5..11,self.inst.bit_range(25..30));
        if self.inst.bit(31) {  self.imm.set_bit_range(12..32,0b1111111111111111111); }
        else { self.imm.set_bit_range(12..32,0b0000000000000000000); }
    }

    fn imm_u(&mut self) {
        self.imm.set_bit_range(0..12,0b000000000000);
        self.imm.set_bit_range(12..32,self.inst.bit_range(12..31));
    }

    fn imm_j(&mut self) {
        self.imm.set_bit(0,false);
        self.imm.set_bit_range(1..11,self.inst.bit_range(21..30));
        self.imm.set_bit(11,self.inst.bit(20));
        self.imm.set_bit_range(12..20,self.inst.bit_range(25..30));
        if self.inst.bit(31) {  self.imm.set_bit_range(20..32,0b111111111111); }
        else { self.imm.set_bit_range(20..32,0b000000000000); }
    }

    fn init_imm(&mut self) {
        if self.is_i() { self.imm_i() }
        else if self.is_s() { self.imm_s() }
        else if self.is_b() { self.imm_b() }
        else if self.is_u() { self.imm_u() }
        else if self.is_j() { self.imm_j() }
    }
    
    

}


fn main() -> Result<()> {
    println!("RSIM Configured!");
    let mut ins_mem = InsMem{
        mem : Vec::new(),
        path : String::from("src/output.bin")
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