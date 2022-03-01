use bit::BitIndex;
pub struct Decoder{
    pub inst : u32,
    pub imm  : u32
}

impl Decoder{
    pub fn reset_imm(&mut self){ self.imm = 0x0 }
    pub fn init_inst(&mut self,inst : u32){ self.inst = inst }

    pub fn is_u(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 5u32) | (val == 13u32) { return true; } 
        false
    }

    pub fn is_j(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if val == 27u32 { return true; } 
        false
    }

    pub fn is_b(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if val == 24u32 { return true; } 
        false
    }

    pub fn is_s(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 8u32) | (val == 9u32) { return true; } 
        false
    }

    pub fn is_r(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 11u32) | (val == 12u32) 
           | (val == 14u32) | (val == 20u32) 
        { return true; } 
        false
    }

    pub fn is_i(&mut self) -> bool{ 
        let val = self.inst.bit_range(2..7);
        if (val == 0u32) | (val == 1u32) 
           | (val == 4u32) | (val == 6u32) 
           | (val == 25u32) 
        { return true; } 
        false
    }

    pub fn rs1(&mut self) -> u32 
    { self.inst.bit_range(15..20) }

    pub fn rs2(&mut self) -> u32 
    { self.inst.bit_range(20..25) }

    pub fn rd(&mut self) -> u32
    { self.inst.bit_range(7..12) }

    pub fn rs1_valid(&mut self) -> bool
    { self.is_r() | self.is_s() | self.is_b() | self.is_i() }

    pub fn rs2_valid(&mut self) -> bool
    { self.is_r() | self.is_s() | self.is_b() }

    pub fn rd_valid(&mut self) -> bool
    { (self.is_r() | self.is_i() | self.is_j() | self.is_u()) & (self.rd() != 0)  }

    pub fn imm_valid(&mut self) -> bool 
    { self.is_s() | self.is_i() | self.is_j() | self.is_u() | self.is_b()  }

    pub fn imm_i(&mut self) {
        self.imm.set_bit_range(0..11,self.inst.bit_range(20..31));
        if self.inst.bit(31) {  self.imm.set_bit_range(11..32,0b11111111111111111111); }
        else { self.imm.set_bit_range(11..32,0b00000000000000000000); }
    }

    pub fn imm_s(&mut self) {
        self.imm.set_bit_range(0..5,self.inst.bit_range(7..12));
        self.imm.set_bit_range(5..11,self.inst.bit_range(25..31));
        if self.inst.bit(31) {  self.imm.set_bit_range(11..32,0b11111111111111111111); }
        else { self.imm.set_bit_range(11..32,0b00000000000000000000); }
    }

    pub fn imm_b(&mut self) {
        self.imm.set_bit(0,false);
        self.imm.set_bit_range(1..5,self.inst.bit_range(8..12));
        self.imm.set_bit_range(5..11,self.inst.bit_range(25..31));
        if self.inst.bit(31) {  self.imm.set_bit_range(12..32,0b1111111111111111111); }
        else { self.imm.set_bit_range(12..32,0b0000000000000000000); }
    }

    pub fn imm_u(&mut self) {
        self.imm.set_bit_range(0..12,0b000000000000);
        self.imm.set_bit_range(12..32,self.inst.bit_range(12..32));
    }

    pub fn imm_j(&mut self) {
        self.imm.set_bit(0,false);
        self.imm.set_bit_range(1..11,self.inst.bit_range(21..30));
        self.imm.set_bit(11,self.inst.bit(20));
        self.imm.set_bit_range(12..20,self.inst.bit_range(25..30));
        if self.inst.bit(31) {  self.imm.set_bit_range(20..32,0b111111111111); }
        else { self.imm.set_bit_range(20..32,0b000000000000); }
    }

    pub fn init_imm(&mut self) {
        if self.is_i() { self.imm_i() }
        else if self.is_s() { self.imm_s() }
        else if self.is_b() { self.imm_b() }
        else if self.is_u() { self.imm_u() }
        else if self.is_j() { self.imm_j() }
    }

    pub fn norm_bit(&mut self) -> u32 {
        let mut dec_bits : u32 = 0u32;
        dec_bits.set_bit_range(0..7,self.inst.bit_range(0..7)); 
        dec_bits.set_bit_range(7..10,self.inst.bit_range(12..15));
        dec_bits.set_bit(10,self.inst.bit(30));
        dec_bits
    }

    pub fn fori_bit(&mut self) -> u32 {
        let mut dec_bits : u32 = 0u32;
        dec_bits.set_bit_range(0..7,self.inst.bit_range(0..7)); 
        dec_bits.set_bit_range(7..10,self.inst.bit_range(12..15));
        dec_bits.set_bit(10,self.imm.bit(10));
        dec_bits
    }
    
    pub fn is_beq(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if (dec_bits == 99u32) || (dec_bits == 1123u32) { return true; }
        false
    }

    pub fn is_bne(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if (dec_bits == 227u32) || (dec_bits == 1251u32) { return true; }
        false
    }

    pub fn is_blt(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if (dec_bits == 611u32) | (dec_bits == 1635u32) { return true; }
        false
    }

    pub fn is_bge(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if (dec_bits == 739u32) || (dec_bits == 1763u32) { return true; }
        false
    }

    pub fn is_bltu(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if (dec_bits == 867u32) || (dec_bits == 1891u32) { return true; }
        false
    }

    pub fn is_bgeu(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if (dec_bits == 995u32) || (dec_bits == 2019u32) { return true; }
        false
    }

    pub fn is_add(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 51u32  { return true; }
        false
    }

    pub fn is_sub(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 1075u32  { return true; }
        false
    }

    pub fn is_sll(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 179u32  { return true; }
        false
    }

    pub fn is_slt(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 307u32  { return true; }
        false
    }

    pub fn is_sltu(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 435u32  { return true; }
        false
    }

    pub fn is_xor(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 563u32  { return true; }
        false
    }

    pub fn is_srl(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 691u32  { return true; }
        false
    }

    pub fn is_sra(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 1715u32  { return true; }
        false
    }

    pub fn is_or(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 819u32  { return true; }
        false
    }

    pub fn is_and(&mut self) -> bool{
        let mut dec_bits = self.norm_bit();
        if dec_bits == 947u32  { return true; }
        false
    }

    pub fn is_lui(&mut self) -> bool{
        let mut dec_bits = self.inst.bit_range(0..7);
        if dec_bits == 55u32  { return true; }
        false
    }

    pub fn is_auipc(&mut self) -> bool{
        let mut dec_bits = self.inst.bit_range(0..7);
        if dec_bits == 23u32  { return true; }
        false
    }

    pub fn is_jal(&mut self) -> bool{
        let mut dec_bits = self.inst.bit_range(0..7);
        if dec_bits == 111u32  { return true; }
        false
    }

    pub fn is_load(&mut self) -> bool {
        let mut dec_bits = self.inst.bit_range(0..7);
        if dec_bits == 3u32  { return true; }
        false
    }

    pub fn is_addi(&mut self) -> bool{
        let mut dec_bits = self.fori_bit();
        if (dec_bits == 19u32) || (dec_bits == 1043u32) { return true; }
        false
    }

    pub fn is_jalr(&mut self) -> bool{
        let mut dec_bits = self.fori_bit();
        if (dec_bits == 103u32) || (dec_bits == 1127u32) { return true; }
        false
    }

    pub fn is_slti(&mut self) -> bool{
        let mut dec_bits = self.fori_bit();
        if (dec_bits == 275u32) || (dec_bits == 1299u32) { return true; }
        false
    }

    pub fn is_sltiu(&mut self) -> bool{
        let mut dec_bits = self.fori_bit();
        if (dec_bits == 403u32) || (dec_bits == 1427u32) { return true; }
        false
    }

    pub fn is_xori(&mut self) -> bool{
        let mut dec_bits = self.fori_bit();
        if (dec_bits == 531u32) || (dec_bits == 1555u32) { return true; }
        false
    }

    pub fn is_ori(&mut self) -> bool{
        let mut dec_bits = self.fori_bit();
        if (dec_bits == 787u32) || (dec_bits == 1811u32) { return true; }
        false
    }

    pub fn is_andi(&mut self) -> bool{
        let mut dec_bits = self.fori_bit();
        if (dec_bits == 915u32) || (dec_bits == 1939u32) { return true; }
        false
    }

    pub fn is_slli(&mut self) -> bool {
        let mut dec_bits = self.fori_bit();
        if dec_bits == 147u32  { return true; }
        false
    }

    pub fn is_srli(&mut self) -> bool {
        let mut dec_bits = self.fori_bit();
        if dec_bits == 659u32  { return true; }
        false
    }

    pub fn is_srai(&mut self) -> bool {
        let mut dec_bits = self.fori_bit();
        if dec_bits == 1683u32  { return true; }
        false
    }
}
