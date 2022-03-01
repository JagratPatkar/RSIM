use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;

pub struct ALU{
    pub result: u32
}

impl ALU{

    fn com_addi(&mut self,dec: &mut Decoder,rf : &mut RefMem){
        self.result = rf.src1 + dec.imm
    }

    fn com_add(&mut self,rf : &mut RefMem){
        self.result = rf.src1 + rf.src2
    }

    pub fn compute(&mut self,dec: &mut Decoder,rf : &mut RefMem,dm : &mut DataMem){
        if dec.is_addi() { self.com_addi(dec,rf) }
        else if dec.is_add() { self.com_add(rf) }
    }
}