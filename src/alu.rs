use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;
use bit::BitIndex;

pub struct ALU{
    pub result: u32
}

impl ALU{

    fn sltu_result(&mut self,rf : &mut RefMem) -> u32{
        let mut t : u32 = 0x0;
        t.set_bit(1,rf.src1 < rf.src2);
        t
    }

    fn sltiu_result(&mut self,rf : &mut RefMem,dec : &mut Decoder) -> u32{
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

    fn sra(&mut self,rf : &mut RefMem) -> u64
    { self.sext_src1(rf) >> rf.src2.bit_range(0..5) }

    fn srai(&mut self,rf : &mut RefMem,dec : &mut Decoder) -> u64
    { self.sext_src1(rf) >> dec.imm.bit_range(0..5) }

    fn lui(&mut self,dec: &mut Decoder) -> u32{ 
        let mut t : u32 = 0;
        t.set_bit_range(12..32,dec.imm.bit_range(12..32));
        t
     }
    
    fn slt(&mut self,rf: &mut RefMem,sd : u32,res: u32) -> u32 {
        if rf.src1.bit(31) == sd.bit(31) { res }
        else {
            let mut t : u32 = 0x0;
            t.set_bit(0,rf.src1.bit(31));
            t 
        }
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
        else if dec.is_sub() { self.result = rf.src1.overflowing_sub(rf.src2).0 }
        else if dec.is_sll() { self.result = rf.src1 << rf.src2.bit_range(0..5) }
        else if dec.is_srl() { self.result = rf.src1 << rf.src2.bit_range(0..5) }
        else if dec.is_sltu() { self.result = self.sltu_result(rf) }
        else if dec.is_sltiu() { self.result = self.sltiu_result(rf,dec) }
        else if dec.is_lui() { self.result = self.lui(dec) }
        else if dec.is_auipc() { self.result = pc.overflowing_add(dec.imm).0 }
        else if dec.is_jal() || dec.is_jalr() { self.result = pc.overflowing_add(0x04).0 }
        else if dec.is_slt() { 
            let t = self.sltu_result(rf);
            self.result = self.slt(rf,rf.src2,t) 
        }
        else if dec.is_sltiu() { 
            let t = self.sltiu_result(rf,dec);
            self.result = self.slt(rf,dec.imm,t) 
        }
        else if dec.is_sra() { self.result = self.sra(rf).bit_range(0..32) as u32 }
        else if dec.is_srai() { self.result = self.srai(rf,dec).bit_range(0..32) as u32 }
        else if dec.is_load() || dec.is_s() {  self.result = rf.src1.overflowing_add(dec.imm).0 }
        else { self.result = 0x0 }
    }
}