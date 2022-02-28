use std::fs::File;
use std::io::{BufReader,Read};
use std::io::Result;
use bit::BitIndex;

pub struct InsMem{
    pub mem : Vec<u8>,
    pub path : String
}

impl InsMem{
    pub fn populate_mem(&mut self) -> Result<()> {
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

    pub fn get_ins(&mut self,addr: u32) -> Option<u32> {
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

    pub fn print_mem(&mut self){
        println!("{:?}",self.mem);
    }
}