use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;
use bit::BitIndex;

pub struct ALU{
    pub result: u32
}

impl ALU{

    fn stlu_result(&mut self,rf : &mut RefMem) -> u32{
        let mut t : u32 = 0x0;
        t.set_bit(1,rf.src1 < rf.src2);
        t
    }

    fn stliu_result(&mut self,rf : &mut RefMem,dec : &mut Decoder) -> u32{
        let mut t : u32 = 0x0;
        t.set_bit(1,(rf.src1 < dec.imm));
        t
    }

    fn sext_src1(&mut self,rf : &mut RefMem) -> u64 { 
        let mut t: u64 = 0x0;
        t.set_bit_range(0..32,rf.src1.into());
        if rf.src1.bit(31) { t.set_bit_range(32..64,u32::MAX.into()); }
        else { t.set_bit_range(32..64,0u64); }
        t
    }

    fn sra_result(&mut self,rf : &mut RefMem) -> u64
    { self.sext_src1(rf) >> rf.src2.bit_range(0..5) }

    fn srai_result(&mut self,rf : &mut RefMem,dec : &mut Decoder) -> u64
    { self.sext_src1(rf) >> dec.imm.bit_range(0..5) }

    pub fn compute(&mut self,dec: &mut Decoder,rf : &mut RefMem,dm : &mut DataMem){
        if dec.is_addi() {  self.result = rf.src1.overflowing_add(dec.imm).0 }
        else if dec.is_add() { self.result = rf.src1.overflowing_add(rf.src2).0 }
        else { self.result = 0x0 }
    }
}