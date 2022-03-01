use crate::decoder::Decoder;


pub struct RefMem {
    pub mem : [u32;32],
    pub src1 : u32,
    pub src2 : u32
 }
 
 impl RefMem{
     pub fn init(&mut self) {
         let mut i = 0;
         loop{
             if i < 32 { 
                 self.mem[i] = i as u32; 
                 i += 1;
             }
             else { break; }
         }
     }
     pub fn compute(&mut self,dec: &mut Decoder){
         if dec.rs1_valid() { self.src1 = self.read(dec.rs1()) }
         if dec.rs2_valid() { self.src2 = self.read(dec.rs2()) }
     }
     pub fn read(&mut self,addr: u32) -> u32{ self.mem[addr as usize] }
     pub fn write(&mut self,res: u32,dec: &mut Decoder){
        
         if dec.rd_valid() { 
             let va = (dec.rd()) as usize;
             self.mem[va] = res; 
         }
     }
     pub fn print_mem(&mut self){ println!("{:?}",self.mem) }
 }