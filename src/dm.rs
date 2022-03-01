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
    // pub fn store(&mut self,dec : &mut Decoder,addr : u32,sd : u32){
    //     if dec.is_s() {
    //         let val = [sd.bit_range(0..8) as u8,sd.bit_range(8..16) as u8,
    //                   sd.bit_range(16..24) as u8,sd.bit_range(24..32) as u8];
    //         let mut i = 0;
    //         let mut va = addr as usize;
    //         loop{
    //             self.mem[va] = val[i];
    //             let p = va as u32;
    //             va = (p + 0x01) as usize;
    //         }
    //     }
    // }

    // pub fn load(&mut self,addr: u32) -> u32{
    //     let mut va = addr as usize;
    //     let mut b : [u8;4] = [0,0,0,0];
    //     let mut i = 0;
    //     loop{
    //         if i < 4 {
    //             b[i] = self.mem[va];
    //             i += 1;
    //             let p = va as u32;
    //             va = (p + 0x01) as usize;
    //         }else { break; }
    //     }
    //     u32::from_be_bytes(b)
    // }

    pub fn print_mem(&mut self){ println!("{:?}",self.mem); }
}
