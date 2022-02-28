use crate::decoder::Decoder;
use crate::dm::DataMem;
use crate::rfm::RefMem;
use crate::alu::ALU;
use crate::im::InsMem;



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
               
                // Send to decoder
                decoder.init_inst(inst);
                decoder.init_imm();
                ref_mem.compute(&mut decoder);
                alu.compute(&mut decoder,&mut ref_mem,&mut d_mem);
                println!("{}",alu.result);
                //ref_mem.print_mem();
                //d_mem.print_mem();
                self.counter = self.counter + 0x04;
            } else{ break; }
        }
    }
}
