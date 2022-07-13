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

    const _ALU_SWTCHA_B1: ControlLine  = 1 << 3;
    const _ALU_SWTCHA_B0: ControlLine  = 1 << 4;
    const _ALU_DIV4B: ControlLine     = 1 << 5;
    const _ALU_OP_B2: ControlLine      = 1 << 6;
    const _ALU_OP_B1: ControlLine      = 1 << 7;
    const _ALU_OP_B0: ControlLine      = 1 << 8;
    
    const _ALU_OUTMOD_B2: ControlLine  = 1 << 9;
    const _ALU_OUTMOD_B1: ControlLine  = 1 << 10;
    const _ALU_OUTMOD_B0: ControlLine  = 1 << 11;
    const AAU_OUT: ControlLine         = 1 << 12;

    const _ALU_OUT_B1: ControlLine     = 1 << 13;

    const _ALU_SWTCHB_REG: ControlLine = 1 << 14;

    const _ALU_OUT_B0: ControlLine     = 1 << 15;

    const AAU_SWTCHMOD: ControlLine   = 1 << 16;

    const CNTRL_IMMOUT: ControlLine   = 1 << 17;

    const IR_NOP: ControlLine         = 1 << 18;

    const RAM_ENO: ControlLine        = 1 << 19;
    const RAM_LOAD: ControlLine       = 1 << 20;

    const CNTRL_UPIMM: ControlLine    = 1 << 21;
    const ROUT_TO_RIN: ControlLine    = 1 << 22;
    const MOUT_TO_RIN: ControlLine    = 1 << 23;

    const REG_OUT: ControlLine        = 1 << 24;

    const _RAM_INSWTCH_B1: ControlLine = 1 << 25;
    const _RAM_INSWTCH_B2: ControlLine = 1 << 26;

    const _RAM_OUTSWTCH_B1: ControlLine = 1 << 27;
    const _RAM_OUTSWTCH_B2: ControlLine = 1 << 28;
    const _RAM_OUTSWTCH_B3: ControlLine = 1 << 29;

    // COMP stands for composed
    const ALU_OP_ADD:    ControlLine   =         0  |         0  |          0;
    const ALU_OP_AND:    ControlLine   =         0  |         0  | _ALU_OP_B0;
    const ALU_OP_XOR:    ControlLine   =         0  | _ALU_OP_B1 |          0;
    const ALU_OP_LLSHFT: ControlLine   =         0  | _ALU_OP_B1 | _ALU_OP_B0;
    const ALU_OP_LRSHFT: ControlLine   = _ALU_OP_B2 |         0  |          0;
    const ALU_OP_ARSHFT: ControlLine   = _ALU_OP_B2 |         0  | _ALU_OP_B0;
    const ALU_OP_SUB:    ControlLine   = _ALU_OP_B2 | _ALU_OP_B1 |          0;
    const ALU_OP_OR:     ControlLine   = _ALU_OP_B2 | _ALU_OP_B1 | _ALU_OP_B0;

    const ALU_OUTMOD_NONE: ControlLine       =              0 |              0 |              0;
    const ALU_OUTMOD_SIGNBIT: ControlLine    =              0 |              0 | _ALU_OUTMOD_B0;
    const ALU_OUTMOD_SUBC: ControlLine       =              0 | _ALU_OUTMOD_B1 |              0;
    const ALU_OUTMOD_MUL4: ControlLine       =              0 | _ALU_OUTMOD_B1 | _ALU_OUTMOD_B0;
    const ALU_OUTMOD_INVSIGNBIT: ControlLine = _ALU_OUTMOD_B2 |              0 |              0;
    const ALU_OUTMOD_INVSUBC: ControlLine    = _ALU_OUTMOD_B2 |              0 | _ALU_OUTMOD_B0;
    const ALU_OUTMOD_ISZR: ControlLine       = _ALU_OUTMOD_B2 | _ALU_OUTMOD_B1 |              0;
    const ALU_OUTMOD_INVISZR: ControlLine    = _ALU_OUTMOD_B2 | _ALU_OUTMOD_B1 | _ALU_OUTMOD_B0;

    const ALU_SWTCHA_REG: ControlLine     =             0  |             0;
    const ALU_SWTCHA_ADDRBUS: ControlLine =             0  | _ALU_SWTCHA_B0;
    const ALU_SWTCHA_REGDIV4: ControlLine = _ALU_SWTCHA_B1 |             0;
    const ALU_SWTCHA_3: ControlLine       = _ALU_SWTCHA_B1 | _ALU_SWTCHA_B0;

    const ALU_SWTCHB_REG: ControlLine     = _ALU_SWTCHB_REG |             0;
    const ALU_SWTCHB_REGDIV4: ControlLine = _ALU_SWTCHB_REG |    _ALU_DIV4B;
    const ALU_SWTCHB_IMM:     ControlLine =               0 |             0;
    const ALU_SWTCHB_IMMDIV4: ControlLine =               0 |    _ALU_DIV4B;

    const ALU_OUT: ControlLine  = _ALU_OUT_B1 | _ALU_OUT_B0;
    const ALU_MOUT: ControlLine = _ALU_OUT_B1 |           0;
    const ALU_AOUT: ControlLine =           0 | _ALU_OUT_B0;

    const RAM_IN_WORD: ControlLine =               0 |               0;
//  const RAM_IN_WORD: ControlLine = _RAM_INSWTCH_B1 |               0;
    const RAM_IN_BYTE: ControlLine =               0 | _RAM_INSWTCH_B2;
    const RAM_IN_HALF: ControlLine = _RAM_INSWTCH_B1 | _RAM_INSWTCH_B2;

    const RAM_OUT_BYTE_SEXT: ControlLine = _RAM_OUTSWTCH_B1 | _RAM_OUTSWTCH_B2 | _RAM_OUTSWTCH_B3;
    const RAM_OUT_BYTE_ZEXT: ControlLine = _RAM_OUTSWTCH_B1 |                0 | _RAM_OUTSWTCH_B3;
    const RAM_OUT_HALF_SEXT: ControlLine =                0 | _RAM_OUTSWTCH_B2 | _RAM_OUTSWTCH_B3;
    const RAM_OUT_HALF_ZEXT: ControlLine =                0 |                0 | _RAM_OUTSWTCH_B3;
    const RAM_OUT_WORD:      ControlLine =                0 |                0 |                0;
 // const RAM_OUT_WORD:      ControlLine = _RAM_OUTSWTCH_B1 | _RAM_OUTSWTCH_B2 |                0;
 // const RAM_OUT_WORD:      ControlLine = _RAM_OUTSWTCH_B1 |                0 |                0;
 // const RAM_OUT_WORD:      ControlLine =                0 | _RAM_OUTSWTCH_B2 |                0;

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

    const OP_IMM:    u8 = 0x04;
    const OP_LUI:    u8 = 0x0D;
    const OP_AUIPC:  u8 = 0x05;
    const OP_OP:     u8 = 0x0C;
    const OP_JAL:    u8 = 0x1B;
    const OP_JALR:   u8 = 0x19;
    const OP_BRANCH: u8 = 0x18;
    const OP_LOAD:   u8 = 0x00;
    const OP_STORE:  u8 = 0x08;
    const OP_FENCE:  u8 = 0x03;

    for ANY7 in 0u8 ..= 2u8.pow(1)-1 {
        r[InstAddr::from_fields_b(0, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_ADD;       // ADDI
        r[InstAddr::from_fields_b(1, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_LLSHFT;    // SLLI  ( shift left logical imm )
        r[InstAddr::from_fields_b(2, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_SIGNBIT | ALU_OP_SUB;       // SLTI  ( set less than imm )
        r[InstAddr::from_fields_b(3, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_SUBC    | ALU_OP_SUB;       // SLTIU ( set less than imm unsigned )
        r[InstAddr::from_fields_b(4, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_XOR;       // XORI
        r[InstAddr::from_fields_b(5,     false, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_LRSHFT;    // SRLI  ( shift right logical imm )
        r[InstAddr::from_fields_b(5,      true, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_ARSHFT;    // SRAI  ( shift right arithmetic imm )
        r[InstAddr::from_fields_b(6, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_OR;        // ORI
        r[InstAddr::from_fields_b(7, ANY7 != 0, OP_IMM)] |= CNTRL_IMMOUT | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_AND;       // ANDI


        r[InstAddr::from_fields_b(0,     false, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_ADD;       // ADD
        r[InstAddr::from_fields_b(0,      true, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_SUB;       // SUB
        r[InstAddr::from_fields_b(1, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_LLSHFT;    // SLL  ( shift left logical )
        r[InstAddr::from_fields_b(2, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_SIGNBIT | ALU_OP_SUB;       // SLT  ( set less than )
        r[InstAddr::from_fields_b(3, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_SUBC    | ALU_OP_SUB;       // SLTU ( set less than unsigned )
        r[InstAddr::from_fields_b(4, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_XOR;       // XOR
        r[InstAddr::from_fields_b(5,     false, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_LRSHFT;    // SRL  ( shift right logical )
        r[InstAddr::from_fields_b(5,      true, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_ARSHFT;    // SRA  ( shift right arithmetic )
        r[InstAddr::from_fields_b(6, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_OR;        // OR
        r[InstAddr::from_fields_b(7, ANY7 != 0, OP_OP)] |= ALU_SWTCHB_REG | ALU_OUT | ALU_OUTMOD_NONE    | ALU_OP_AND;       // AND

        for ANY3 in 0u8 ..= 2u8.pow(3)-1 {
            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_LUI)]   |= CNTRL_IMMOUT | CNTRL_UPIMM | ROUT_TO_RIN; // LUI
            // Note: the convention is that the 
            // output of the alu after any operation on addresses is always going to be a native address
            // to get the address as byte-oriented do ALU_OUTMOD_MUL4
            // This convention was put in place to make it easier for JAL to jump outside the first 4gb in hardware

            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_AUIPC)] |= CNTRL_IMMOUT | CNTRL_UPIMM | ALU_SWTCHA_ADDRBUS | ALU_SWTCHB_IMMDIV4 | ALU_OUT | ALU_OUTMOD_MUL4 | ALU_OP_ADD; // AUIPC
        
            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_JAL)]  = (CNTRL_IMMOUT | ALU_SWTCHA_ADDRBUS | PC_OUT | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc jump addr */) | ( ALU_MOUT | PC_LOAD | PC_ENC | IR_NOP /* load jump addr into pc, inducing pipeline stall */) | (AAU_OUT /* store ret addr in rd register */); // JAL
            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_JALR)] = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | PC_OUT | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc jump addr */) | ( ALU_MOUT | PC_LOAD | PC_ENC | IR_NOP /* load jump addr into pc, inducing pipeline stall */) | (AAU_OUT /* store ret addr in rd register */); // JALR
            r[InstAddr::from_fields_b(ANY3, ANY7 != 0, OP_FENCE)] = 0;
        }


        r[InstAddr::from_fields_b(0, ANY7 != 0, OP_BRANCH)] = (CNTRL_IMMOUT | AAU_SWTCHMOD | PC_OUT /* calc branch addr */) | ( (PC_ENC | PC_LOAD) | AAU_OUT | IR_NOP /* load branch addr into pc, inducing pipeline stall */) | (ALU_SWTCHB_REG | ALU_OUTMOD_ISZR        | ALU_OP_XOR | ALU_OUT /* calc wether to branch or not. this is used by AAU */); // BEQ
        r[InstAddr::from_fields_b(1, ANY7 != 0, OP_BRANCH)] = (CNTRL_IMMOUT | AAU_SWTCHMOD | PC_OUT /* calc branch addr */) | ( (PC_ENC | PC_LOAD) | AAU_OUT | IR_NOP /* load branch addr into pc, inducing pipeline stall */) | (ALU_SWTCHB_REG | ALU_OUTMOD_INVISZR     | ALU_OP_XOR | ALU_OUT /* calc wether to branch or not. this is used by AAU */); // BNE

        r[InstAddr::from_fields_b(4, ANY7 != 0, OP_BRANCH)] = (CNTRL_IMMOUT | AAU_SWTCHMOD | PC_OUT /* calc branch addr */) | ( (PC_ENC | PC_LOAD) | AAU_OUT | IR_NOP /* load branch addr into pc, inducing pipeline stall */) | (ALU_SWTCHB_REG | ALU_OUTMOD_SIGNBIT    | ALU_OP_SUB | ALU_OUT /* calc wether to branch or not, this is used by AAU */); // BLT
        r[InstAddr::from_fields_b(5, ANY7 != 0, OP_BRANCH)] = (CNTRL_IMMOUT | AAU_SWTCHMOD | PC_OUT /* calc branch addr */) | ( (PC_ENC | PC_LOAD) | AAU_OUT | IR_NOP /* load branch addr into pc, inducing pipeline stall */) | (ALU_SWTCHB_REG | ALU_OUTMOD_INVSIGNBIT | ALU_OP_SUB | ALU_OUT /* calc wether to branch or not, this is used by AAU */); // BGE
        r[InstAddr::from_fields_b(6, ANY7 != 0, OP_BRANCH)] = (CNTRL_IMMOUT | AAU_SWTCHMOD | PC_OUT /* calc branch addr */) | ( (PC_ENC | PC_LOAD) | AAU_OUT | IR_NOP /* load branch addr into pc, inducing pipeline stall */) | (ALU_SWTCHB_REG | ALU_OUTMOD_SUBC       | ALU_OP_SUB | ALU_OUT /* calc wether to branch or not, this is used by AAU */); // BLTU
        r[InstAddr::from_fields_b(7, ANY7 != 0, OP_BRANCH)] = (CNTRL_IMMOUT | AAU_SWTCHMOD | PC_OUT /* calc branch addr */) | ( (PC_ENC | PC_LOAD) | AAU_OUT | IR_NOP /* load branch addr into pc, inducing pipeline stall */) | (ALU_SWTCHB_REG | ALU_OUTMOD_INVSUBC    | ALU_OP_SUB | ALU_OUT /* calc wether to branch or not, this is used by AAU */); // BGEU


        r[InstAddr::from_fields_b(0, ANY7 != 0, OP_LOAD)]  = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_ENO | RAM_OUT_BYTE_ZEXT | MOUT_TO_RIN); // LB
        r[InstAddr::from_fields_b(1, ANY7 != 0, OP_LOAD)]  = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_ENO | RAM_OUT_HALF_ZEXT | MOUT_TO_RIN); // LH
        r[InstAddr::from_fields_b(2, ANY7 != 0, OP_LOAD)]  = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_ENO | RAM_OUT_WORD      | MOUT_TO_RIN); // LW

        r[InstAddr::from_fields_b(4, ANY7 != 0, OP_LOAD)]  = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_ENO | RAM_OUT_BYTE_SEXT | MOUT_TO_RIN); // LBU
        r[InstAddr::from_fields_b(5, ANY7 != 0, OP_LOAD)]  = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_ENO | RAM_OUT_HALF_SEXT | MOUT_TO_RIN); // LHU


        r[InstAddr::from_fields_b(2, ANY7 != 0, OP_STORE)] = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_LOAD | RAM_IN_WORD | REG_OUT); // SW
        r[InstAddr::from_fields_b(1, ANY7 != 0, OP_STORE)] = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_LOAD | RAM_IN_HALF | REG_OUT); // SH
        r[InstAddr::from_fields_b(0, ANY7 != 0, OP_STORE)] = (CNTRL_IMMOUT | ALU_SWTCHA_REGDIV4 | ALU_SWTCHB_IMMDIV4 | ALU_OP_ADD /* calc load addr */) | (ALU_AOUT | IR_NOP /* make memory go to that addr */) | (RAM_LOAD | RAM_IN_BYTE | REG_OUT); // SB

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
