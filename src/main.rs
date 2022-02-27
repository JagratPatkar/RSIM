use std::fs::File;
use std::io::{BufReader,Read};
use std::io::Result;

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
                println!("{}",x);
                self.counter = self.counter + 0x04
                // Send to decoder
            } else{ break; }
        }
    }
}

struct Decoder{

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