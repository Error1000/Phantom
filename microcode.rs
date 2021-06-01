#[derive(Debug, Clone, Copy)]
struct InstAddr{
    data: u16
}

impl Default for InstAddr{
    fn default() -> Self{
        InstAddr{data: 0}
    }
}
impl InstAddr {
    pub fn from_fields(funct3: u8, funct7: bool, high_bit: bool, low_nibble: u8) -> Self {
        InstAddr{
            data:  ((funct3 as u16) << 6) | ((funct7 as u16) << 5) | ((high_bit as u16) << 4) | (low_nibble as u16)
        }
    }

    pub fn from_fields_b(funct3: u8, funct7: bool, opcode: u8) -> Self{
        InstAddr{
            data:  ((funct3 as u16) << 6) | ((funct7 as u16) << 5) | (opcode as u16)
        }  
    }

    pub fn get_low_nibble(&self) -> u16 {
        ( self.data & 0b000001111 ) >> 0
    }

    pub fn get_high_bit(&self) -> u16 {
        (self.data & 0b000010000) >> 4
    }

    pub fn get_opcode(&self) -> u16 {
        (self.get_high_bit() << 4) | self.get_low_nibble()
    }

    pub fn get_funct7_state(&self) -> u16 {
        ((self.data & 0b000100000) >> 5)
    }

    pub fn get_funct3(&self) -> u16 {
        (self.data & 0b111000000) >> 6
    }

}

type ControlLine = u32;


    const PC_ENC: ControlLine         = 1 << 0;
    const PC_LOAD: ControlLine        = 1 << 1;
    const PC_OUT: ControlLine         = 1 << 2;

    const ALU_SWTCHA_B1: ControlLine  = 1 << 3;
    const ALU_SWTCHA_B0: ControlLine  = 1 << 4;
    const ALU_DIV4B: ControlLine     = 1 << 5;
    const ALU_OP_B2: ControlLine      = 1 << 6;
    const ALU_OP_B1: ControlLine      = 1 << 7;
    const ALU_OP_B0: ControlLine      = 1 << 8;
    
    const ALU_OUTMOD_B1: ControlLine  = 1 << 9;
    const ALU_OUTMOD_B0: ControlLine  = 1 << 10;
    const AAU_OUT: ControlLine  = 1 << 11;

    const ALU_OUT: ControlLine        = 1 << 12;

    const ALU_SWTCHB_REG: ControlLine = 1 << 13;

    const ALU_MOUT: ControlLine       = 1 << 14;

    const AAU_SWTCHMOD: ControlLine   = 1 << 15;

    const CNTRL_IMMOUT: ControlLine   = 1 << 16;

    const IR_NOP: ControlLine         = 1 << 17;

    const RAM_ENO: ControlLine        = 1 << 18;
    const RAM_LOAD: ControlLine       = 1 << 19;

    const CNTRL_UPIMM: ControlLine    = 1 << 20;
    const ROUT_TO_RIN: ControlLine    = 1 << 21;

    const ALU_OP_ADD:   ControlLine   =         0 |         0 |         0;
    const ALU_OP_AND:   ControlLine   =         0 |         0 | ALU_OP_B0;
    const ALU_OP_XOR:   ControlLine   =         0 | ALU_OP_B1 |         0;
    const ALU_OP_LLSHFT: ControlLine  =         0 | ALU_OP_B1 | ALU_OP_B0;
    const ALU_OP_RSHFT: ControlLine   = ALU_OP_B2 |         0 |         0;
    const ALU_OP_SUB:   ControlLine   = ALU_OP_B2 |         0 | ALU_OP_B0;
    const ALU_OP_OR:     ControlLine  = ALU_OP_B2 | ALU_OP_B1 |         0;
    const ALU_OP_7:     ControlLine   = ALU_OP_B2 | ALU_OP_B1 | ALU_OP_B0;

    const ALU_OUTMOD_NONE: ControlLine    =             0 |             0;
    const ALU_OUTMOD_SIGNBIT: ControlLine =             0 | ALU_OUTMOD_B0;
    const ALU_OUTMOD_SUBC: ControlLine    = ALU_OUTMOD_B1 |             0;
    const ALU_OUTMOD_MUL4: ControlLine    = ALU_OUTMOD_B1 | ALU_OUTMOD_B0;

    const ALU_SWTCHA_REG: ControlLine     =             0 |             0;
    const ALU_SWTCHA_ADDRBUS: ControlLine =             0 | ALU_SWTCHA_B0;
    const ALU_SWTCHA_REGDIV4: ControlLine = ALU_SWTCHA_B1 |             0;
    const ALU_SWTCHA_3: ControlLine       = ALU_SWTCHA_B1 | ALU_SWTCHA_B0;

#[derive(Debug)]
struct Rom {
    data: [ControlLine; 0b1_000_000_000] // 9 bits of address
}
impl Rom {
    pub fn new() -> Self{
        Rom{
            data: [Default::default(); 0b1_000_000_000]
        }
    }
}
use std::ops::{Index, IndexMut};

impl Index<InstAddr> for Rom {
    type Output = ControlLine;
    fn index(&self, ind: InstAddr) -> &Self::Output {
        &self.data[ind.data as usize]
    }
}

impl IndexMut<InstAddr> for Rom {
    fn index_mut(&mut self, ind: InstAddr) -> &mut Self::Output {
        &mut self.data[ind.data as usize]
    }
}


fn main(){
    let mut r = Rom::new();

    // LOAD control lines
    for addr in 0 ..= 2usize.pow(9)-1 {
        r.data[addr as usize] |= PC_ENC | PC_OUT | RAM_ENO;
    }

    // Integer Register-Immediate Computational Instructions
    const OP_IMM:    u8 = 0x04;
    const OP_LUI:    u8 = 0x0D;
    const OP_AUIPC:  u8 = 0x05;
    const OP_OP:     u8 = 0x0C;
    const OP_JAL:    u8 = 0x1B;
    const OP_JALR:   u8 = 0x19;
    const OP_BRANCH: u8 = 0x18;

    for ANY7 in 0u8 ..= 2u8.pow(1)-1 {
        r[InstAddr::from_fields_b(0, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_ADD;       // ADDI
        r[InstAddr::from_fields_b(1, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_LLSHFT;    // SLLI  ( shift left logical imm )
        r[InstAddr::from_fields_b(2, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_SIGNBIT | ALU_OP_SUB;       // SLTI  ( set less than imm )
        r[InstAddr::from_fields_b(3, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_SUBC    | ALU_OP_SUB;       // SLTIU ( set less than imm unsigned )
        r[InstAddr::from_fields_b(4, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_XOR;       // XORI
        r[InstAddr::from_fields_b(5, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_RSHFT;     // SRLI  ( shift right logical imm )
        r[InstAddr::from_fields_b(5, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_RSHFT;     // SRAI  ( shift right arithmetic imm )
        r[InstAddr::from_fields_b(6, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_OR;        // ORI
        r[InstAddr::from_fields_b(7, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_AND;       // ANDI


        r[InstAddr::from_fields_b(0, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_ADD;       // ADD
        r[InstAddr::from_fields_b(1, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_LLSHFT;    // SLL  ( shift left logical )
        r[InstAddr::from_fields_b(2, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_SIGNBIT | ALU_OP_SUB;       // SLT  ( set less than )
        r[InstAddr::from_fields_b(3, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_SUBC    | ALU_OP_SUB;       // SLTU ( set less than unsigned )
        r[InstAddr::from_fields_b(4, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_XOR;       // XOR
        r[InstAddr::from_fields_b(5, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_RSHFT;     // SRL  ( shift right logical )
        r[InstAddr::from_fields_b(5, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_RSHFT;     // SRA  ( shift right arithmetic )
        r[InstAddr::from_fields_b(6, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_OR;        // OR
        r[InstAddr::from_fields_b(7, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_AND;       // AND

        for ANY3 in 0u8 ..= 2u8.pow(3)-1 {
            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_LUI)]   |= CNTRL_IMMOUT | CNTRL_UPIMM | ROUT_TO_RIN; // LUI
            // Note: the convention is that the 
            // output of the alu after any operation on addresses is always going to be a native address
            // to get the address as byte-oriented do ALU_OUTMOD_MUL4
            // This convention was put in place to make it easier for JAL to jump outside the first 4gb in hardware

            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_AUIPC)] |= CNTRL_IMMOUT | CNTRL_UPIMM | ALU_SWTCHA_ADDRBUS | ALU_DIV4B | ALU_OUT | ALU_OUTMOD_MUL4 | ALU_OP_ADD; // AUIPC
        
            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_JAL)]  = CNTRL_IMMOUT | ALU_SWTCHA_ADDRBUS | ALU_DIV4B | ALU_MOUT | ALU_OP_ADD | PC_LOAD | PC_OUT | PC_ENC | IR_NOP | AAU_OUT; // JAL
            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_JALR)] = CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_DIV4B | ALU_MOUT | ALU_OP_ADD | PC_LOAD | PC_OUT | PC_ENC | IR_NOP | AAU_OUT; // JALR
        }
        r[InstAddr::from_fields_b(4, ANY7 != 0, OP_BRANCH)] = CNTRL_IMMOUT | AAU_SWTCHMOD | PC_ENC | PC_LOAD | PC_OUT | IR_NOP | ALU_SWTCHB_REG | ALU_OUTMOD_SIGNBIT | ALU_OP_SUB | ALU_OUT | AAU_OUT; // BLT
        r[InstAddr::from_fields_b(6, ANY7 != 0, OP_BRANCH)] = CNTRL_IMMOUT | AAU_SWTCHMOD | PC_ENC | PC_LOAD | PC_OUT | IR_NOP | ALU_SWTCHB_REG | ALU_OUTMOD_SUBC    | ALU_OP_SUB | ALU_OUT | AAU_OUT; // BLTU

    }

   // DOWN-CLOCK: SWITCH IRS & LOAD in IR 1 & PARSE in IR 2
   // UP-CLOCK: EXECUTE in IR 2

    use std::io::Write;
    let mut fout = std::fs::File::create("rom.bin").expect("Opening out file!");
    // Write to file
    for addr in 0 ..= 2usize.pow(9)-1 {
        let b1 = ((r.data[addr] & 0x000000FF) >> 8*0) as u8;
        let b2 = ((r.data[addr] & 0x0000FF00) >> 8*1) as u8;
        let b3 = ((r.data[addr] & 0x00FF0000) >> 8*2) as u8;
        let b4 = ((r.data[addr] & 0xFF000000) >> 8*3) as u8;
        fout.write_all(&[b1, b2, b3, b4]).unwrap();
    }
}