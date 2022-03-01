use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;

pub struct ALU{
    pub result: u32
}

impl ALU{

    pub fn compute(&mut self,dec: &mut Decoder,rf : &mut RefMem,dm : &mut DataMem){
        if dec.is_addi() { self.result = rf.src1 + dec.imm }
        else if dec.is_add() { self.result = rf.src1 + rf.src2 }
        else { self.result = 0x0 }
    }
}