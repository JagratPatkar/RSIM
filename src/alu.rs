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

    fn lui(&mut self,dec: &mut Decoder) -> u32{ 
        let mut t : u32 = 0;
        t.set_bit_range(12..32,dec.imm.bit_range(12..32));
        t
     }
    

    pub fn compute(&mut self,dec: &mut Decoder,rf : &mut RefMem,dm : &mut DataMem,pc: u32){
        if dec.is_addi() {  self.result = rf.src1.overflowing_add(dec.imm).0 }
        else if dec.is_add() { self.result = rf.src1.overflowing_add(rf.src2).0 }
        else if dec.is_andi() { self.result = rf.src1 & dec.imm }
        else if dec.is_ori() { self.result = rf.src1 | dec.imm }
        else if dec.is_xori() { self.result = rf.src1 ^ dec.imm }
        else if dec.is_slli() { self.result = rf.src1 << dec.imm.bit_range(0..6) }
        else if dec.is_srli() { self.result = rf.src1 >> dec.imm.bit_range(0..6) }
        else if dec.is_and() { self.result = rf.src1 & rf.src2 }
        else if dec.is_or() { self.result = rf.src1 | rf.src2 }
        else if dec.is_xor() { self.result = rf.src1 ^ rf.src2 }
        else if dec.is_sll() { self.result = rf.src1 << rf.src2.bit_range(0..5) }
        else if dec.is_srl() { self.result = rf.src1 << rf.src2.bit_range(0..5) }
        else if dec.is_sltu() { self.result = self.stlu_result(rf) }
        else if dec.is_sltiu() { self.result = self.stliu_result(rf,dec) }
        else if dec.is_lui() { self.result = self.lui(dec) }
        else if dec.is_auipc() { self.result = pc.overflowing_add(dec.imm).0 }
        else if dec.is_jal() || dec.is_jalr() { self.result = pc.overflowing_add(0x04).0 }
        else { self.result = 0x0 }
    }
}