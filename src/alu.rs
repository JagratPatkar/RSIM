use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;

pub struct ALU{
    pub result: u32
}

impl ALU{

    pub fn compute(&mut self,dec: &mut Decoder,rf : &mut RefMem,dm : &mut DataMem){
        if dec.is_addi() {  self.result = rf.src1.overflowing_add(dec.imm).0 }
        else if dec.is_add() { self.result = rf.src1.overflowing_add(rf.src2).0 }
        else { self.result = 0x0 }
    }
}