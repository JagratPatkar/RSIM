use crate::decoder::Decoder;
use bit::BitIndex;


pub struct DataMem{
    pub mem : [u8; 32*4]
}

impl DataMem{
    pub fn store(&mut self,addr : u32,dec : &mut Decoder,sd : u32){
        if dec.is_s() {
            let val = [sd.bit_range(24..32) as u8,sd.bit_range(16..24) as u8,
                      sd.bit_range(8..16) as u8,sd.bit_range(0..8) as u8];
            let mut i = 0;
            let mut va = addr as usize;
            loop{
                if i < 4 {
                    self.mem[va] = val[i];
                    let p = va as u32;
                    va = (p + 0x01) as usize;
                    i += 1;
                }
                else { break; }
            }
        }
    }

    pub fn load(&mut self,addr: u32) -> u32{
        let mut va = addr as usize;
        let mut b : [u8;4] = [0,0,0,0];
        let mut i = 0;
        loop{
            if i < 4 {
                b[i] = self.mem[va];
                i += 1;
                let p = va as u32;
                va = (p + 0x01) as usize;
            }else { break; }
        }
        u32::from_be_bytes(b)
    }

    pub fn print_mem(&mut self){ 
        let mut addr : u32 = 0x0;
        let mut ma : [u32; 32] = [0; 32];
        let mut j = 0;
        loop{
            if j < 32 {
                let mut va = addr as usize;
                let mut b : [u8;4] = [0,0,0,0];
                let mut i = 0;
                loop{
                    if i < 4 {
                        b[i] = self.mem[va];
                        i += 1;
                        let p = va as u32;
                        va = (p + 0x01) as usize;
                    }else { break; }
                }
                ma[j] = u32::from_be_bytes(b);
                j += 1;
                addr = addr.overflowing_add(0x04).0
            }
            else { break; }
        }
        
        println!("{:?}",ma); 
    }
}
