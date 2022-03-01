use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;
use crate::alu::ALU;
use crate::im::InsMem;
use bit::BitIndex;



pub struct PC{
    pub counter : u32,
    pub reset_val : u32
}

impl PC{
    pub fn start_fetch(&mut self,mem : &mut InsMem){
        let mut decoder = Decoder{
            inst: 0x0,
            imm : 0x0
        };
        let mut ref_mem  = RefMem{
            mem : [0; 32],
            src1 : 0,
            src2 : 0
        };
        ref_mem.init();
        let mut d_mem = DataMem{
            mem : [0; 32*4]
        };
        let mut alu = ALU{
            result: 0x0
        };
        loop{
            if let Some(x) = mem.get_ins(self.counter) {
                let inst = x;
                decoder.init_inst(inst);
                decoder.reset_imm();
                decoder.init_imm();
                ref_mem.compute(&mut decoder);
                alu.compute(&mut decoder,&mut ref_mem,self.counter);
                d_mem.store(alu.result.bit_range(0..5),&mut decoder,ref_mem.src2);
                let l_data  = d_mem.load(alu.result.bit_range(0..5));
                let res = if decoder.is_load() { l_data } else { alu.result };
                ref_mem.write(res,&mut decoder);
                self.next(&mut decoder,&mut ref_mem);
            } else{ 
                ref_mem.print_mem(); 
                d_mem.print_mem();
                break;
            }
        }
    }

    fn next(&mut self,dec : &mut Decoder,rf: &mut RefMem){
        if self.is_valid_br(dec,rf) || dec.is_j() {  self.counter = self.counter.overflowing_add(dec.imm).0; }
        else if dec.is_jalr() { self.counter = rf.src1.overflowing_add(dec.imm).0 } 
        else { self.counter = self.counter.overflowing_add(0x04).0; }
    }

    fn is_valid_br(&mut self,dec : &mut Decoder,rf: &mut RefMem) -> bool{
        if dec.is_beq() { rf.src1 == rf.src2 }
        else if dec.is_bne() { rf.src1 != rf.src2 }
        else if dec.is_blt() { (rf.src1 < rf.src2) || (rf.src1.bit(31) != rf.src2.bit(31)) }
        else if dec.is_bge() { (rf.src1 >= rf.src2) || (rf.src1.bit(31) != rf.src2.bit(31)) }
        else if dec.is_bltu() { rf.src1 < rf.src2 }
        else if dec.is_bgeu() { rf.src1 >= rf.src2 }
        else { false }
    }
}
