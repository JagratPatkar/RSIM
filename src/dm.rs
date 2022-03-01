use crate::decoder::Decoder;
use bit::BitIndex;


pub struct DataMem{
    pub mem : [u32; 32]
}

impl DataMem{
    pub fn load(&mut self,addr: u32) -> u32{  self.mem[addr as usize]  }
    pub fn store(&mut self,addr: u32,dec: &mut Decoder,d: u32){
        if dec.is_s() { 
            let va = addr as usize;
            self.mem[va] = d; 
        }
    }

    pub fn print_mem(&mut self){ println!("{:?}",self.mem); }
}
